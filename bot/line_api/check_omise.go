package main

import (
	"encoding/json"
	"log"
	"os"
	"strings"

	"github.com/aws/aws-lambda-go/events"
	"github.com/kkdai/line-social-sdk-go"
)

func checkOmise(request events.APIGatewayProxyRequest) (string, error) {

	param := &struct {
		CharaID     string `json:"charaId"`
		AccessToken string `json:"accessToken"`
	}{}
	if err := json.Unmarshal([]byte(request.Body), param); err != nil {
		return "", err
	}

	bot, err := NewLine(
		os.Getenv(cebab2Camel(param.CharaID)+"_CHANNEL_SECRET"),
		os.Getenv(cebab2Camel(param.CharaID)+"_CHANNEL_TOKEN"),
		os.Getenv(cebab2Camel(param.CharaID)+"_STAFF_GROUP_ID"),
		os.Getenv(cebab2Camel(param.CharaID)+"_ORDER_GROUP_ID"),
	)
	if err != nil {
		log.Println(err)
		return "", err
	}
	log.Printf("chara_id %s", cebab2Camel(param.CharaID))
	log.Printf("channel_id %s", cebab2Camel(param.CharaID)+"_CHANNEL_ID")
	log.Printf("channel_secret %s", cebab2Camel(param.CharaID)+"_CHANNEL_SECRET")
	log.Printf("access_token %s", param.AccessToken)

	client, err := social.New(
		os.Getenv(cebab2Camel(param.CharaID)+"_CHANNEL_ID"),
		os.Getenv(cebab2Camel(param.CharaID)+"_CHANNEL_SECRET"),
	)
	if err != nil {
		log.Println(err)
		return "", err
	}

	// ユーザ名取得
	name := "誰か"
	if param.AccessToken != "" {
		res, err := client.GetUserProfile(param.AccessToken).Do()
		if err != nil {
			log.Println(err)
			return "", err
		}
		name = res.DisplayName
	}

	// xxさんがお店情報を見ています
	bot.StaffTextMessage(name + "さんがお店情報を開いたよ")

	return "{}", nil
}

func cabab2Snake(s string) string {
	s = strings.ReplaceAll(s, ".", "_")
	s = strings.ReplaceAll(s, "/", "_")
	s = strings.ToUpper(s)
	return s
}
func cebab2Camel(s string) string {
	var cc string
	s = strings.ReplaceAll(s, ".", "/")
	for _, w := range strings.Split(s, "/") {
		cc += strings.ToUpper(string(w[0])) + w[1:]
	}
	return cc
}
