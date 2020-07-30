package main

import (
	"log"
	"strings"

	"github.com/line/line-bot-sdk-go/linebot"
)

type Line struct {
	bot          *linebot.Client
	staffGroupID string
	orderGroupID string
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
	return r.PushMessageToStaffRoom(linebot.NewTextMessage(message))
}
func (r *Line) TemplateMessageToStaffRoom(altText string, template linebot.Template) error {
	return r.PushMessageToStaffRoom(
		linebot.NewTextMessage(altText),
		linebot.NewTemplateMessage(altText, template),
	)
}

func (r *Line) PushMessageToStaffRoom(message ...linebot.SendingMessage) error {
	if len(r.staffGroupID) > 0 {
		if _, err := r.bot.PushMessage(r.staffGroupID, message...).Do(); err != nil {
			log.Printf("staff push message Error: %v", err)
			return err
		}
	}
	return nil
}

func (r *Line) TextMessageToOrderRoom(message string) error {
	return r.PushMessageToOrderRoom(linebot.NewTextMessage(message))
}
func (r *Line) TemplateMessageToOrderRoom(altText string, template linebot.Template) error {
	return r.PushMessageToOrderRoom(
		linebot.NewTextMessage(altText),
		linebot.NewTemplateMessage(altText, template),
	)
}

func (r *Line) PushMessageToOrderRoom(message ...linebot.SendingMessage) error {
	if len(r.orderGroupID) > 0 {
		if _, err := r.bot.PushMessage(r.orderGroupID, message...).Do(); err != nil {
			log.Printf("order push message Error: %v", err)
			return err
		}
	}
	return nil
}

/*
func (r *Line) withSender() *linebot.Sender {
	return &linebot.Sender{
				Name:    r.displayName,
				IconURL: r.iconURL,
	}
}
*/

func ClientID(omiseURI string) string {
	return strings.Split(omiseURI, "/")[0]
}
func OmiseID(omiseURI string) string {
	return strings.Split(omiseURI, "/")[1]
}
