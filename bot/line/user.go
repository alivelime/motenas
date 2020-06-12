package main

import (
	"encoding/json"
	"log"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/lambda"

	"github.com/line/line-bot-sdk-go/linebot"
)

type Event struct {
	Text    string `json:"text"`
	CharaID string `json:"chara_id"`
	ID      string `json:"id"`
	App     string `json:"app"`
}
type Message struct {
	Type     string     `json:"type"`
	Message  string     `json:"message"`
	Carousel []Carousel `json:"carousel,omitempty"`
}
type Carousel struct {
	Image string `json:"image"`
	URL   string `json:"url"`
	Title string `json:"title"`
	Text  string `json:"text"`
}

func handleTextChara(r *Line, message *linebot.TextMessage, replyToken, userID string) {
	// redirect staff
	prof, err := r.Bot.GetProfile(userID).Do()
	if err != nil {
		log.Printf("can't get prof: %v", err)
		return
	}
	r.StaffTextMessage(userID + "\n" + prof.DisplayName + "\n" + message.Text)

	payload, _ := json.Marshal(Event{
		Text:    message.Text,
		CharaID: r.Name,
		ID:      userID, App: "line"})

	res, err := lambda.New(session.New()).Invoke(&lambda.InvokeInput{
		FunctionName:   aws.String(ARN + "omoinas-dev-ukekotae"),
		Payload:        payload,
		InvocationType: aws.String("RequestResponse"),
	})
	if err != nil {
		log.Println(err)
	}

	var result Message
	_ = json.Unmarshal(res.Payload, &result)

	switch result.Type {
	case "message":
		r.ReplyTextMessage(replyToken, result.Message)
		r.StaffTextMessage(result.Message)
	case "carousel":
		log.Printf("%#v", result.Carousel)
		var columns []*linebot.CarouselColumn
		for _, c := range result.Carousel {
			columns = append(columns, r.NewCarouselColumn(c.Image, c.Title, c.Text, linebot.NewURIAction("商品情報を見る", c.URL)))
		}
		r.ReplyTemplateMessage(replyToken, result.Message, r.NewCarouselTemplate(columns...))
		r.StaffTemplateMessage(result.Message, r.NewCarouselTemplate(columns...))
	default:
		break
	}
}
