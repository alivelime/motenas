package main

import (
	"strings"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-lambda-go/lambdacontext"
)

const (
	ARN = "arn:aws:lambda:ap-northeast-1:591658611168:function:"
)

func Handler(request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {

	headers := map[string]string{
		"Content-Type":                    "application/json",
		"Access-Control-Allow-Origin":     request.Headers["Origin"],
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
	default:
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
