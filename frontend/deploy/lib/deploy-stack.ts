import * as cdk from '@aws-cdk/core';
import * as cloudfront from '@aws-cdk/aws-cloudfront'
import * as s3 from '@aws-cdk/aws-s3'
import * as s3deploy from '@aws-cdk/aws-s3-deployment'
import * as iam from '@aws-cdk/aws-iam'

export class DeployStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, stage: string, props?: cdk.StackProps) {
    if (!stage) {
      stage = 'dev'
    }
    super(scope, `id-${stage}`, props);

    const websiteBucket = new s3.Bucket(this, `KikinasuLiffBucket-${stage}`, {
      websiteErrorDocument: 'index.html',
      websiteIndexDocument: 'index.html',
    });

    const websiteIdentity = new cloudfront.OriginAccessIdentity(
      this,
      `KikinasuLiffIdentity-${stage}`,
    );

    const webSiteBucketPolicyStatement = new iam.PolicyStatement({
      actions: ['s3:GetObject'],
      effect: iam.Effect.ALLOW,
      principals: [
        websiteIdentity.grantPrincipal,
      ],
      resources: [`${websiteBucket.bucketArn}/*`],
    });

    websiteBucket.addToResourcePolicy(webSiteBucketPolicyStatement);

    const websiteDistribution = new cloudfront.CloudFrontWebDistribution(
      this,
      `KikinasuLiffDistribution-${stage}`,
      {
        errorConfigurations: [
          {
            errorCachingMinTtl: 300,
            errorCode: 403,
            responseCode: 200,
            responsePagePath: '/index.html',
          },
          {
            errorCachingMinTtl: 300,
            errorCode: 404,
            responseCode: 200,
            responsePagePath: '/index.html',
          },
        ],
        originConfigs: [
          {
            s3OriginSource: {
              s3BucketSource: websiteBucket,
              originAccessIdentity: websiteIdentity,
            },
            behaviors: [
              {
                isDefaultBehavior: true,
              },
            ],
          },
        ],
        priceClass: cloudfront.PriceClass.PRICE_CLASS_ALL,
      },
    );

    new s3deploy.BucketDeployment(this, `KikinasuLiffDeploy-${stage}`, {
      sources: [s3deploy.Source.asset('../richmenu/build')],
      destinationBucket: websiteBucket,
      distribution: websiteDistribution,
      distributionPaths: ['/*'],
    });
  }
}
