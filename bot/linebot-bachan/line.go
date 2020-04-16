package main

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/lambda"

	"github.com/line/line-bot-sdk-go/linebot"
)

type Line struct {
	ChannelSecret string
	ChannelToken  string
	Bot           *linebot.Client
}

func NewLine(secret, token string) (Line, error) {
	bot, err := linebot.New(
		secret,
		token,
	)
	if err != nil {
		return Line{}, err
	}

	return Line{
		ChannelSecret: secret,
		ChannelToken:  token,
		Bot:           bot,
	}, nil
}

func (r *Line) SendTextMessage(replyToken, message string) error {
	return r.Reply(replyToken, linebot.NewTextMessage(message))
}

func (r *Line) SendTemplateMessage(replyToken, altText string, template linebot.Template) error {
	return r.Reply(replyToken,
		linebot.NewTextMessage(altText),
		linebot.NewTemplateMessage(altText, template),
	)
}

func (r *Line) Reply(replyToken string, message ...linebot.SendingMessage) error {
	if _, err := r.Bot.ReplyMessage(replyToken, message...).Do(); err != nil {
		fmt.Printf("Reply Error: %v", err)
		return err
	}
	return nil
}

func (r *Line) NewCarouselColumn(thumbnailImageURL, title, text string, actions ...linebot.TemplateAction) *linebot.CarouselColumn {
	return &linebot.CarouselColumn{
		ThumbnailImageURL: thumbnailImageURL,
		Title:             title,
		Text:              text,
		Actions:           actions,
	}
}

func (r *Line) NewCarouselTemplate(columns ...*linebot.CarouselColumn) *linebot.CarouselTemplate {
	return &linebot.CarouselTemplate{
		Columns: columns,
	}
}

func (r *Line) EventRouter(eve []*linebot.Event) {
	for _, event := range eve {
		switch event.Type {
		case linebot.EventTypeMessage:
			switch message := event.Message.(type) {
			case *linebot.TextMessage:
				r.handleText(message, event.ReplyToken, event.Source.UserID)
			}
		}
	}
}

type Event struct {
	Text  string `json:"text"`
	Chara string `json:"chara"`
	ID    string `json:"id"`
	App   string `json:"app"`
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

func (r *Line) handleText(message *linebot.TextMessage, replyToken, userID string) {
	payload, _ := json.Marshal(Event{Text: message.Text, Chara: "bachan", ID: userID, App: "line"})

	res, err := lambda.New(session.New()).Invoke(&lambda.InvokeInput{
		FunctionName:   aws.String("arn:aws:lambda:ap-northeast-1:591658611168:function:omoinas-dev-ukekotae"),
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
		r.SendTextMessage(replyToken, result.Message)
	case "carousel":
		log.Printf("%#v", result.Carousel)
		var columns []*linebot.CarouselColumn
		for _, c := range result.Carousel {
			columns = append(columns, r.NewCarouselColumn(c.Image, c.Title, c.Text, linebot.NewURIAction("商品情報を見る", c.URL)))
		}
		r.SendTemplateMessage(replyToken, result.Message, r.NewCarouselTemplate(columns...))
	default:
		break
	}
}
