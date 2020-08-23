package main

import (
	"log"
	"strings"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-lambda-go/lambdacontext"
)

const (
	ARN = "arn:aws:lambda:ap-northeast-1:591658611168:function:"
)

func Handler(request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {

	log.Printf("request header : ", request.Headers["origin"])
	headers := map[string]string{
		"Content-Type":                    "application/json",
		"Access-Control-Allow-Origin":     request.Headers["origin"],
		"Access-Control-Allow-Methods":    "OPTIONS,POST,GET",
		"Access-Control-Allow-Headers":    "Origin,Authorization,Accept,X-Requested-With",
		"Access-Control-Allow-Credential": "true",
	}
	if request.HTTPMethod == "OPTIONS" {
		return events.APIGatewayProxyResponse{
			Headers:    headers,
			Body:       "",
			StatusCode: 200,
		}, nil
	}

	var res string
	var err error

	fn := strings.Split(lambdacontext.FunctionName, "line")[1]
	switch fn {
	case "CheckOmise":
		res, err = checkOmise(request)
	case "SetOmise":
		res, err = setOmise(request)
	case "EnterOmise":
		res, err = enterOmise(request)
	default:
		log.Printf("call %s", fn)
		return events.APIGatewayProxyResponse{
			Headers:    headers,
			Body:       "no method",
			StatusCode: 404,
		}, err
	}

	return events.APIGatewayProxyResponse{
		Headers:    headers,
		Body:       res,
		StatusCode: 200,
	}, err
}

func main() {
	lambda.Start(Handler)
}
