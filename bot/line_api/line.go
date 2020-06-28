package main

import (
	"log"
	"strings"

	"github.com/line/line-bot-sdk-go/linebot"
)

type Line struct {
	mainBot     *linebot.Client
	displayName string
	iconURL     string

	bot          *linebot.Client
	staffGroupID string
	orderGroupID string
}

func NewLine(name, icon, secret, token, msecret, mtoken, staffGroupID, orderGroupID string) (Line, error) {
	bot, err := linebot.New(
		secret,
		token,
	)
	if err != nil {
		return Line{}, err
	}
	mainBot, err := linebot.New(
		msecret,
		mtoken,
	)
	if err != nil {
		return Line{}, err
	}

	return Line{
		mainBot:     mainBot,
		displayName: name,
		iconURL:     icon,

		bot:          bot,
		staffGroupID: staffGroupID,
		orderGroupID: orderGroupID,
	}, nil
}

func (r *Line) BroadcastTextMessage(message string) error {
	return r.Broadcast(linebot.NewTextMessage(message))
}
func (r *Line) Broadcast(message ...linebot.SendingMessage) error {
	if _, err := r.bot.BroadcastMessage(message...).Do(); err != nil {
		log.Printf("Broadcast Error: %v", err)
		return err
	}
	return nil
}

func (r *Line) PushTextMessage(to, message string) error {
	return r.PushMessage(to, linebot.NewTextMessage(message))
}
func (r *Line) PushMessage(to string, message ...linebot.SendingMessage) error {
	if _, err := r.bot.PushMessage(to, message...).Do(); err != nil {
		log.Printf("Push Message Error: %v", err)
		return err
	}
	return nil
}

func (r *Line) TextMessageToStaffRoom(message string) error {
	return r.PushMessageToStaffRoom(linebot.NewTextMessage(message).WithSender(r.withSender()))
}
func (r *Line) TemplateMessageToStaffRoom(altText string, template linebot.Template) error {
	return r.PushMessageToStaffRoom(
		linebot.NewTextMessage(altText).WithSender(r.withSender()),
		linebot.NewTemplateMessage(altText, template).WithSender(r.withSender()),
	)
}

func (r *Line) PushMessageToStaffRoom(message ...linebot.SendingMessage) error {
	if len(r.staffGroupID) > 0 {
		if _, err := r.mainBot.PushMessage(r.staffGroupID, message...).Do(); err != nil {
			log.Printf("staff push message Error: %v", err)
			return err
		}
	}
	return nil
}

func (r *Line) withSender() *linebot.Sender {
	return &linebot.Sender{
		Name:    r.displayName,
		IconURL: r.iconURL,
	}
}

func ClientID(charaID string) string {
	return strings.Split(charaID, "/")[0]
}
func OmiseID(charaID string) string {
	return strings.Split(charaID, "/")[1]
}
