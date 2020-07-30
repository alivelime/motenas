package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/lambda"
	"github.com/kkdai/line-social-sdk-go"
)

func enterOmise(request events.APIGatewayProxyRequest) (string, error) {
	accessToken := strings.TrimPrefix(request.Headers["Authorization"], "Bearer: ")
	if accessToken == "" {
		// 401
		return "", errors.New("access token is empty.")
	}

	param := &struct {
		OmiseURI string `json:"omiseUri"`
		ID       string `json:"id"`

		//  ラムダ用パラメータ
		ClientID  string `json:"clientId"`
		OmiseID   string `json:"omiseId"`
		MaroudoID string `json:"maroudoId"`
	}{}
	if err := json.Unmarshal([]byte(request.Body), param); err != nil {
		return "", err
	}
	omise := cebab2Camel(param.OmiseURI)

	bot, err := NewLine(
		os.Getenv(omise+"_CHANNEL_SECRET"),
		os.Getenv(omise+"_CHANNEL_TOKEN"),
		os.Getenv(omise+"_STAFF_GROUP_ID"),
		os.Getenv(omise+"_ORDER_GROUP_ID"),
	)
	if err != nil {
		log.Println(err)
		return "", err
	}

	client, err := social.New(
		os.Getenv(omise+"_CHANNEL_ID"),
		os.Getenv(omise+"_CHANNEL_SECRET"),
	)
	if err != nil {
		log.Println(err)
		return "", err
	}

	// ユーザ名取得
	prof, err := client.GetUserProfile(accessToken).Do()
	if err != nil {
		log.Println(err)
		return "", err
	}
	name := prof.DisplayName

	param.MaroudoID = prof.UserID
	param.ClientID = ClientID(param.OmiseURI)
	param.OmiseID = OmiseID(param.OmiseURI)

	payload, _ := json.Marshal(param)
	res, err := lambda.New(session.New()).Invoke(&lambda.InvokeInput{
		FunctionName:   aws.String(ARN + "omoinas-" + os.Getenv("ENV") + "-createDenpyo"),
		Payload:        payload,
		InvocationType: aws.String("RequestResponse"),
	})
	if err != nil {
		log.Println(err)
		return "", err
	}

	result := struct {
		OK      bool   `json:"ok"`
		Message string `json:"message"`
	}{}
	_ = json.Unmarshal(res.Payload, &result)
	if !result.OK {
		log.Println(result.Message)
		return "", errors.New(result.Message)
	}

	bot.PushTextMessage(prof.UserID, fmt.Sprintf("%sさんいらっしゃいませ。", name))
	bot.TextMessageToOrderRoom(fmt.Sprintf("%sさんが入店しました\n伝票番号: %s", name, param.ID))

	return "{}", nil
}
