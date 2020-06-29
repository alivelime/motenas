#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { DeployStack } from '../lib/deploy-stack';

const app = new cdk.App();
const stage = app.node.tryGetContext('stage')
new DeployStack(app, 'DeployStack', stage, {env: {account: '591658611168', region: 'ap-northeast-1'}});
