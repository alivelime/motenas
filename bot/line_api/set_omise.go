package main

import (
	"encoding/json"
	"errors"
	"log"
	"os"
	"strings"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/lambda"
	"github.com/kkdai/line-social-sdk-go"
)

func setOmise(request events.APIGatewayProxyRequest) (string, error) {
	accessToken := strings.TrimPrefix(request.Headers["Authorization"], "Bearer: ")
	if accessToken == "" {
		// 401
		return "", errors.New("access token is empty.")
	}

	param := &struct {
		OmiseURI string `json:"omiseUri"`

		Namae string `json:"namae"`
		Ima   []struct {
			Namae  string `json:"namae"`
			Status string `json:"status"`
		} `json:"ima"`
		Hitokoto   string   `json:"hitokoto"`
		KefuKara   int      `json:"kefuKara"`
		KefuMade   int      `json:"kefuMade"`
		Omotenashi []string `json:"omotenashi"`
		Oshiharai  []string `json:"oshiharai"`
		Yotei      string   `json:"yotei"`
		Oshirase   string   `json:"oshirase"`
		HP         string   `json:"hp"`
		Twitter    string   `json:"twitter"`
		Facebook   string   `json:"facebook"`
		Instagram  string   `json:"instagram"`
		Line       string   `json:"line"`

		Postcode int    `json:"postcode"`
		Prefcode int    `json:"prefcode"`
		City     string `json:"city"`
		Street   string `json:"street"`
		Building string `json:"building"`

		//  ラムダ用パラメータ
		ClientID string `json:"clientId"`
		OmiseID  string `json:"omiseId"`
		UserID   string `json:"userId"`
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

	param.UserID = prof.UserID
	param.ClientID = ClientID(param.OmiseURI)
	param.OmiseID = OmiseID(param.OmiseURI)

	payload, _ := json.Marshal(param)
	res, err := lambda.New(session.New()).Invoke(&lambda.InvokeInput{
		FunctionName:   aws.String(ARN + "omoinas-" + os.Getenv("ENV") + "-setOmise"),
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

	bot.TextMessageToStaffRoom(name + "さんがお店情報を更新したよ")

	return "{}", nil
}
