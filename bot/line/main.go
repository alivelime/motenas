package main

import (
	"errors"
	"fmt"
	"os"
	"strings"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-lambda-go/lambdacontext"
	"github.com/line/line-bot-sdk-go/linebot"
)

const (
	ARN = "arn:aws:lambda:ap-northeast-1:591658611168:function:"
)

func Handler(request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	chara := strings.Split(lambdacontext.FunctionName, "line")[1]

	line, err := NewLine(
		os.Getenv(chara+"_CHANNEL_SECRET"),
		os.Getenv(chara+"_CHANNEL_TOKEN"),
		os.Getenv(chara+"_CHARA_NAME"),
		os.Getenv(chara+"_STAFF_GROUP_ID"),
		os.Getenv(chara+"_ORDER_GROUP_ID"),
	)
	if err != nil {
		fmt.Println(err)
	}
	eve, err := ParseRequest(line.ChannelSecret, request)
	if err != nil {
		status := 200
		if err == linebot.ErrInvalidSignature {
			status = 400
		} else {
			status = 500
		}
		return events.APIGatewayProxyResponse{StatusCode: status}, errors.New("Bad Request")
	}
	line.EventRouter(eve)
	return events.APIGatewayProxyResponse{Body: request.Body, StatusCode: 200}, nil
}

func main() {
	lambda.Start(Handler)
}
