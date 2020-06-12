package main

import (
	"log"

	"github.com/line/line-bot-sdk-go/linebot"
)

type Line struct {
	ChannelSecret string
	ChannelToken  string
	Bot           *linebot.Client
	StaffGroupID  string
	OrderGroupID  string
}

func NewLine(secret, token, staffGroupID, orderGroupID string) (Line, error) {
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
		StaffGroupID:  staffGroupID,
		OrderGroupID:  orderGroupID,
	}, nil
}

func (r *Line) BroadcastTextMessage(message string) error {
	return r.Broadcast(linebot.NewTextMessage(message))
}
func (r *Line) Broadcast(message ...linebot.SendingMessage) error {
	if _, err := r.Bot.BroadcastMessage(message...).Do(); err != nil {
		log.Printf("Broadcast Error: %v", err)
		return err
	}
	return nil
}

func (r *Line) PushTextMessage(to, message string) error {
	return r.PushMessage(to, linebot.NewTextMessage(message))
}
func (r *Line) PushMessage(to string, message ...linebot.SendingMessage) error {
	if _, err := r.Bot.PushMessage(to, message...).Do(); err != nil {
		log.Printf("Push Message Error: %v", err)
		return err
	}
	return nil
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
			log.Printf("staff push message Error: %v", err)
			return err
		}
	}
	return nil
}
