package main

import (
	"fmt"
	"log"
	"strings"

	"github.com/line/line-bot-sdk-go/linebot"
)

type Line struct {
	mainBot     *linebot.Client
	displayName string
	iconURL     string

	ChannelSecret string
	ChannelToken  string
	bot           *linebot.Client
	charaID       string
	staffGroupID  string
	orderGroupID  string
	richMenuID    string
}

func NewLine(display, icon, secret, token, msecret, mtoken, charaID, staffGroupID, orderGroupID, richMenuID string) (Line, error) {
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
		displayName: display,
		iconURL:     icon,

		ChannelSecret: secret,
		ChannelToken:  token,
		bot:           bot,
		charaID:       charaID,
		staffGroupID:  staffGroupID,
		orderGroupID:  orderGroupID,
		richMenuID:    richMenuID,
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
	if _, err := r.bot.ReplyMessage(replyToken, message...).Do(); err != nil {
		fmt.Printf("Reply Error: %v", err)
		return err
	}
	return nil
}
func (r *Line) BroadcastTextMessage(message string) error {
	return r.Broadcast(linebot.NewTextMessage(message))
}
func (r *Line) Broadcast(message ...linebot.SendingMessage) error {
	if _, err := r.bot.BroadcastMessage(message...).Do(); err != nil {
		fmt.Printf("Broadcast Error: %v", err)
		return err
	}
	return nil
}

func (r *Line) PushTextMessage(to, message string) error {
	return r.PushMessage(to, linebot.NewTextMessage(message))
}
func (r *Line) PushMessage(to string, message ...linebot.SendingMessage) error {
	if _, err := r.bot.PushMessage(to, message...).Do(); err != nil {
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
			fmt.Printf("push to staff room Error: %v", err)
			return err
		}
	}
	return nil
}

func (r *Line) TextMessageToOrderRoom(message string) error {
	return r.PushMessageToOrderRoom(linebot.NewTextMessage(message).WithSender(r.withSender()))
}
func (r *Line) TemplateMessageToOrderRoom(altText string, template linebot.Template) error {
	return r.PushMessageToOrderRoom(
		linebot.NewTextMessage(altText).WithSender(r.withSender()),
		linebot.NewTemplateMessage(altText, template).WithSender(r.withSender()),
	)
}
func (r *Line) PushMessageToOrderRoom(message ...linebot.SendingMessage) error {
	if len(r.orderGroupID) > 0 {
		if _, err := r.mainBot.PushMessage(r.orderGroupID, message...).Do(); err != nil {
			fmt.Printf("push to staff room Error: %v", err)
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
				case r.staffGroupID:
				case r.orderGroupID:
				default:
					handleTextChara(r, message, event.ReplyToken, event.Source.UserID)
				}
			}
		case linebot.EventTypeJoin:
			log.Printf("Join group id : %v", event.Source.GroupID)
		case linebot.EventTypeMemberJoined:
			if event.Source.GroupID == r.orderGroupID {
				if event.Joined != nil {
					// handleMemberJoined(r, event.Joined.Members)
				} else {
					handleMemberJoined(r, event.Members, event.ReplyToken)
				}
			}
		case linebot.EventTypeMemberLeft:
			log.Printf("%#v", event)
			if event.Source.GroupID == r.orderGroupID {
				if event.Left != nil {
					// handleMemberLeft(r, event.Left.Members)
				} else {
					handleMemberLeft(r, event.Members)
				}
			}
		}
	}
}

func (r *Line) LinkRichMenu(users []string) {
	_, err := r.bot.BulkLinkRichMenu(r.richMenuID, users...).Do()
	if err != nil {
		log.Println(err)
	}
}
func (r *Line) UnlinkRichMenu(users []string) {
	_, err := r.bot.BulkUnlinkRichMenu(users...).Do()
	if err != nil {
		log.Println(err)
	}
}

func (r *Line) ClientID() string {
	return strings.Split(r.charaID, "/")[0]
}
func (r *Line) OmiseID() string {
	return strings.Split(r.charaID, "/")[1]
}
func (r *Line) CharaName() string {
	return strings.Split(r.charaID, "/")[2]
}
func (r *Line) withSender() *linebot.Sender {
	return &linebot.Sender{
		Name:    r.displayName,
		IconURL: r.iconURL,
	}
}
