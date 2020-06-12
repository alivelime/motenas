package main

import (
	"strings"

	"github.com/line/line-bot-sdk-go/linebot"
)

func handleTextStaff(r *Line, message *linebot.TextMessage, replyToken, userID string) {
	t := strings.Split(message.Text, "\n")
	switch {
	case t[0] == "皆さんへ" || t[0] == "みなさんへ":
		if err := r.BroadcastTextMessage(strings.Join(t[1:], "^n")); err != nil {
			r.ReplyTextMessage(replyToken, "エラーが発生しました "+err.Error())
			return
		}
		r.ReplyTextMessage(replyToken, "全体メッセージを送信しました")
	case t[0] == "お客様へ":
		if err := r.PushTextMessage(t[1], strings.Join(t[2:], "^n")); err != nil {
			r.ReplyTextMessage(replyToken, "エラーが発生しました "+err.Error())
			return
		}
		r.ReplyTextMessage(replyToken, "個別メッセージを送信しました")
	}
}
