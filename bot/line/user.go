package main

import (
	"encoding/json"
	"log"
	"os"

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
	prof, err := r.bot.GetProfile(userID).Do()
	if err != nil {
		log.Printf("can't get prof: %v", err)
		return
	}
	r.TextMessageToStaffRoom(userID + "\n" + prof.DisplayName + "\n" + message.Text)

	payload, _ := json.Marshal(Event{
		Text:    message.Text,
		CharaID: r.charaID,
		ID:      userID, App: "line"})

	res, err := lambda.New(session.New()).Invoke(&lambda.InvokeInput{
		FunctionName:   aws.String(ARN + "omoinas-" + os.Getenv("ENV") + "-ukekotae"),
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
		r.TextMessageToStaffRoom(result.Message)
	case "carousel":
		log.Printf("%#v", result.Carousel)
		var columns []*linebot.CarouselColumn
		for _, c := range result.Carousel {
			columns = append(columns, r.NewCarouselColumn(c.Image, c.Title, c.Text, linebot.NewURIAction("商品情報を見る", c.URL)))
		}
		r.ReplyTemplateMessage(replyToken, result.Message, r.NewCarouselTemplate(columns...))
		r.TemplateMessageToStaffRoom(result.Message, r.NewCarouselTemplate(columns...))
	default:
		break
	}
}

func handleMemberJoined(r *Line, users []linebot.EventSource) {
	type MemberEvent struct {
		ClientID string   `json:"client_id"`
		OmiseID  string   `json:"omise_id"`
		Command  string   `json:"command"`
		Tanamono []string `json:"tanamono"`
	}
	userIDs := make([]string, len(users))
	for _, u := range users {
		userIDs = append(userIDs, u.UserID)
	}
	r.LinkRichMenu(userIDs)

	payload, _ := json.Marshal(MemberEvent{
		ClientID: r.ClientID(),
		OmiseID:  r.OmiseID(),
		Command:  "add",
		Tanamono: userIDs,
	})

	_, err := lambda.New(session.New()).Invoke(&lambda.InvokeInput{
		FunctionName:   aws.String(ARN + "omoinas-" + os.Getenv("ENV") + "-tanamono"),
		Payload:        payload,
		InvocationType: aws.String("RequestResponse"),
	})
	if err != nil {
		log.Println(err)
	}

	r.TextMessageToOrderRoom("こんにちは")
}

func handleMemberLeft(r *Line, users []linebot.EventSource) {
	type MemberEvent struct {
		ClientID string   `json:"client_id"`
		OmiseID  string   `json:"omise_id"`
		Command  string   `json:"command"`
		Tanamono []string `json:"tanamono"`
	}
	userIDs := make([]string, len(users))
	for _, u := range users {
		userIDs = append(userIDs, u.UserID)
	}
	r.LinkRichMenu(userIDs)

	payload, _ := json.Marshal(MemberEvent{
		Command:  "remove",
		Tanamono: userIDs,
	})

	_, err := lambda.New(session.New()).Invoke(&lambda.InvokeInput{
		FunctionName:   aws.String(ARN + "omoinas-" + os.Getenv("ENV") + "-tanamono"),
		Payload:        payload,
		InvocationType: aws.String("RequestResponse"),
	})
	if err != nil {
		log.Println(err)
	}
}
