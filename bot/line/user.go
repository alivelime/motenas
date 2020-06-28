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

func (r *Line) handleTextChara(message *linebot.TextMessage, replyToken, userID string) {
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

func (r *Line) handleMemberJoined(users []*linebot.EventSource, replyToken string) {
	type MemberEvent struct {
		ClientID string   `json:"client_id"`
		OmiseID  string   `json:"omise_id"`
		Command  string   `json:"command"`
		Tanamono []string `json:"tanamono"`
	}
	userIDs := make([]string, len(users))
	for i, u := range users {
		userIDs[i] = u.UserID
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

	r.ReplyTextMessage(replyToken, "よろしくね")
}

func (r *Line) handleMemberLeft(users []*linebot.EventSource) {
	type MemberEvent struct {
		ClientID string   `json:"client_id"`
		OmiseID  string   `json:"omise_id"`
		Command  string   `json:"command"`
		Tanamono []string `json:"tanamono"`
	}
	userIDs := make([]string, len(users))
	for i, u := range users {
		userIDs[i] = u.UserID
	}
	r.UnlinkRichMenu(userIDs)

	payload, _ := json.Marshal(MemberEvent{
		ClientID: r.ClientID(),
		OmiseID:  r.OmiseID(),
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

func (r *Line) handlePostback(replyToken, userID, data string) {
	switch data {
	case "how_to":
		r.ReplyTextMessage(replyToken, "メニューの真ん中のボタンでお店の混雑状況が確認できるよ\n\n何があるの?\n とか xxってない?\n とか聞いてみてね。\n今日やってる? 対しては実装中だよ。")

		// redirect staff
		var message string
		prof, err := r.bot.GetProfile(userID).Do()
		if err != nil {
			log.Printf("can't get prof: %v", err)
			message = "誰かが「できること」を見たよ。(プロフが取得できませんでした)"
		} else {
			if isDev() {
				message = userID + "\n" + prof.DisplayName + "\n" + "さんが「できること」を見たよ"
			} else {
				message = prof.DisplayName + "さんが「できること」を見たよ"
			}
		}

		r.TextMessageToStaffRoom(message)
	case "how_to_omise":
		r.ReplyTextMessage(replyToken, "メニューの「お店管理」から今日の営業時間と混み具合を入れてください。\n「お店について」ボタンから実際のページを確認できます。")

	}
}

func (r *Line) handleFollow(userID string) {
	// redirect staff
	var message string
	prof, err := r.bot.GetProfile(userID).Do()
	if err != nil {
		log.Printf("can't get prof: %v", err)
		message = "誰かが友達になったよ。(プロフが取得できませんでした)"
	} else {
		if isDev() {
			message = userID + "\n" + prof.DisplayName + "\n" + "さんが友達になったよ"
		} else {
			message = prof.DisplayName + "さんが友達になったよ"
		}
	}

	r.TextMessageToStaffRoom(message)
}
func (r *Line) handleUnfollow(userID string) {
	// redirect staff
	// GetProfile()はエラーとなるので使わないこと
	var message string
	if isDev() {
		message = userID + "\nさんがブロックしたよ"
	} else {
		message = "残念なことに、誰かさんからブロックされてしまいました。"
	}

	r.TextMessageToStaffRoom(message)
}
