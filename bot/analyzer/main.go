package main

import (
	"context"
	"encoding/json"
	"log"

	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	service "github.com/aws/aws-sdk-go/service/lambda"

	"cloud.google.com/go/language/apiv1"
	"google.golang.org/api/option"
	languagepb "google.golang.org/genproto/googleapis/cloud/language/v1"
)

type Event struct {
	Text string `json:"text"`
}
type Response struct {
	Message string `json:"message"`
	Ok      bool   `json:"ok"`
}

func Handler(event Event) (Response, error) {
	ctx := context.Background()

	client, err := language.NewClient(ctx, option.WithCredentialsJSON([]byte(credential)))

	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	req := &languagepb.AnalyzeSyntaxRequest{
		Document: &languagepb.Document{
			Type: 1,
			Source: &languagepb.Document_Content{
				Content: event.Text,
			},
			Language: "ja",
		},
		EncodingType: languagepb.EncodingType_UTF8,
	}

	syntax, err := client.AnalyzeSyntax(ctx, req)
	if err != nil {
		log.Fatalf("Failed to AnalyzeSyntax: %v", err)
	}

	/*
		entities, err := client.AnalyzeEntities(ctx, &languagepb.AnalyzeEntitiesRequest{
			Document: &languagepb.Document{
				Source: &languagepb.Document_Content{
					Content: event.Text,
				},
				Type:     languagepb.Document_PLAIN_TEXT,
				Language: "ja",
			},
			EncodingType: languagepb.EncodingType_UTF8,
		})
		log.Printf("%#v", entities)
	*/

	payload, _ := json.Marshal(Event{Text: syntax.Sentences[0].Text.Content})

	message, _ := service.New(session.New()).Invoke(&service.InvokeInput{
		FunctionName:   aws.String("arn:aws:lambda:ap-northeast-1:bttm68tsn0:function:{reply}"),
		Payload:        payload,
		InvocationType: aws.String("RequestResponse"),
	})

	res, _ := json.Marshal(message)
	return Response{
		Message: string(res),
		Ok:      true,
	}, nil
}

func main() {
	lambda.Start(Handler)
}
