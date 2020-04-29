package line

import (
	"fmt"

	"github.com/line/line-bot-sdk-go/linebot"
)

type Line struct {
	ChannelSecret      string
	ChannelToken       string
	Bot                *linebot.Client
	AdminChannelSecret string
	AdminChannelToken  string
	AdminBot           *linebot.Client
}

func NewLine(secret, token, adminSecret, adminToken string) (Line, error) {
	bot, err := linebot.New(
		secret,
		token,
	)
	if err != nil {
		return Line{}, err
	}

	adminBot, err := linebot.New(
		adminSecret,
		adminToken,
	)
	if err != nil {
		return Line{}, err
	}

	return Line{
		ChannelSecret:      secret,
		ChannelToken:       token,
		Bot:                bot,
		AdminChannelSecret: adminSecret,
		AdminChannelToken:  adminToken,
		AdminBot:           adminBot,
	}, nil
}

func (r *Line) SendTextMessage(message string) error {
	return r.Broadcast(linebot.NewTextMessage(message))
}

func (r *Line) SendTemplateMessage(altText string, template linebot.Template) error {
	return r.Broadcast(
		linebot.NewTextMessage(altText),
		linebot.NewTemplateMessage(altText, template),
	)
}

func (r *Line) Broadcast(message ...linebot.SendingMessage) error {
	if _, err := r.Bot.BroadcastMessage(message...).Do(); err != nil {
		fmt.Printf("Broadcast Error: %v", err)
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
				r.SendTextMessage(message.Text)
			}
		}
	}
}
