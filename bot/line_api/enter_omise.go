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
		// 引数
		ID       string `json:"id"`
		ClientID string `json:"clientId"`
		OmiseID  string `json:"omiseId"`

		//  ラムダ用パラメータ
		MaroudoID string `json:"maroudoId"`
	}{}
	if err := json.Unmarshal([]byte(request.Body), param); err != nil {
		return "", err
	}
	omise := cebab2Camel(omiseURI(param.ClientID, param.OmiseID))

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

	payload, _ := json.Marshal(param)
	_, err = lambda.New(session.New()).Invoke(&lambda.InvokeInput{
		FunctionName:   aws.String(ARN + "omoinas-" + os.Getenv("ENV") + "-createDenpyo"),
		Payload:        payload,
		InvocationType: aws.String("RequestResponse"),
	})
	if err != nil {
		log.Println(err)
		return "", err
	}

	bot.PushTextMessage(prof.UserID, fmt.Sprintf("お薬を登録したよ\n", param.ID))
	bot.TextMessageToOrderRoom(fmt.Sprintf("%sさんがお薬を読み取ったよ\n%s", name, param.ID))

	return "{}", nil
}
