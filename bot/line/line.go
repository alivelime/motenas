package main

import (
	"fmt"
	"log"

	"github.com/line/line-bot-sdk-go/linebot"
)

type Line struct {
	ChannelSecret string
	ChannelToken  string
	Bot           *linebot.Client
	Name          string
	StaffGroupID  string
	OrderGroupID  string
}

func NewLine(secret, token, name, staffGroupID, orderGroupID string) (Line, error) {
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
		Name:          name,
		StaffGroupID:  staffGroupID,
		OrderGroupID:  orderGroupID,
	}, nil
}

func (r *Line) ReplyTextMessage(replyToken, message string) error {
	return r.Reply(replyToken, linebot.NewTextMessage(message))
}

func (r *Line) ReplyTemplateMessage(replyToken, altText string, template linebot.Template) error {
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
func (r *Line) BroadcastTextMessage(message string) error {
	return r.Broadcast(linebot.NewTextMessage(message))
}
func (r *Line) Broadcast(message ...linebot.SendingMessage) error {
	if _, err := r.Bot.BroadcastMessage(message...).Do(); err != nil {
		fmt.Printf("Broadcast Error: %v", err)
		return err
	}
	return nil
}

func (r *Line) PushTextMessage(to, message string) error {
	return r.PushMessage(to, linebot.NewTextMessage(message))
}
func (r *Line) PushMessage(to string, message ...linebot.SendingMessage) error {
	if _, err := r.Bot.PushMessage(to, message...).Do(); err != nil {
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

func (r *Line) StaffTextMessage(message string) error {
	return r.StaffPushMessage(linebot.NewTextMessage(message))
}
func (r *Line) StaffTemplateMessage(altText string, template linebot.Template) error {
	return r.StaffPushMessage(
		linebot.NewTextMessage(altText),
		linebot.NewTemplateMessage(altText, template),
	)
}

func (r *Line) StaffPushMessage(message ...linebot.SendingMessage) error {
	if len(r.StaffGroupID) > 0 {
		if _, err := r.Bot.PushMessage(r.StaffGroupID, message...).Do(); err != nil {
			fmt.Printf("Reply Error: %v", err)
			return err
		}
	}
	return nil
}

func (r *Line) EventRouter(eve []*linebot.Event) {
	for _, event := range eve {
		switch event.Type {
		case linebot.EventTypeMessage:
			switch message := event.Message.(type) {
			case *linebot.TextMessage:
				switch event.Source.GroupID {
				case r.StaffGroupID:
				case r.OrderGroupID:
				default:
					handleTextChara(r, message, event.ReplyToken, event.Source.UserID)
				}
			}
		case linebot.EventTypeJoin:
			log.Printf("Join group id : %v", event.Source.GroupID)
		}
	}
}
