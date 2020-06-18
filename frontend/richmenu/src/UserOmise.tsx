import React, { useState } from 'react'
import {
  useParams,
} from 'react-router-dom';

import { makeStyles } from '@material-ui/core/styles';
import Grid from '@material-ui/core/Grid';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import Paper from '@material-ui/core/Paper';
import Divider from '@material-ui/core/Divider';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import {
  faCoffee,
  faRestroom,
  faSmokingBan,
  faSmoking,
} from '@fortawesome/free-solid-svg-icons'


interface RouteParams {
    env: string,
    clientId: string,
    omiseId: string,
    charaId: string,
}

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  paper: {
    padding: theme.spacing(2),
    color: theme.palette.text.secondary,
  },
  title: {
    textAlign: 'center',
    fontWeight: 'bold',
  },
  subhead: {
    textAlign: 'center',
  },
  head: {
    fontSize: "1rem",
    fontWeight: 'bold',
  },
  omiseIcon: {
    fontSize: "2rem",
    margin: '0 0.2rem',
  },
}));


function UserOmise() {
  const {env, clientId, omiseId, charaId} = useParams<RouteParams>();

  liff.ready.then(() => {
    let accessToken = ""
    if (!liff.isLoggedIn()) {
      // liff.login({})
    } else {
       accessToken = liff.getAccessToken()
    }
    let url = `${process.env.REACT_APP_LINE_API_HOST}/${env}/line-api/omise/check`;
    fetch(url, {
      method: "POST",
      mode: "cors",
      cache: "no-cache",
      body: JSON.stringify({
        accessToken: accessToken,
        charaId: `${clientId}/${omiseId}/${charaId}`,
      })
    });

  })

  const classes = useStyles();
  const encodedAddress = encodeURIComponent("東京都文京区千駄木2-33-8")
  return (
    <Grid container className={classes.root} spacing={3}>
      <Grid item xs={12} justify="center">
        <Paper variant="outlined" elevation={3} className={classes.paper}>
          <Typography variant="h1" className={classes.title}>コンフル千駄木店</Typography>
        </Paper>
      </Grid>
      <Grid item xs={12} className={classes.subhead}>
        <Typography variant="h3">今日やってる?</Typography>
      </Grid>
      <Grid item xs={12}>
        <Paper variant="outlined" elevation={3} className={classes.paper}>
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>本日の営業時間</p>
            </Grid>
            <Grid item xs={8}>
              <p>10:00 〜 22:00</p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>混み具合</p>
            </Grid>
            <Grid item xs={8}>
              <p>混んでるよ</p>
            </Grid>
          </Grid>
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>一言</p>
            </Grid>
            <Grid item xs={8}>
              <p>喫煙席は空いてます</p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>その他</p>
            </Grid>
            <Grid item xs={8}>
              <p>
                <span className={classes.omiseIcon}><FontAwesomeIcon icon={faCoffee} /></span>
                <span className={classes.omiseIcon}><FontAwesomeIcon icon={faRestroom} /></span>
                <span className={classes.omiseIcon}><FontAwesomeIcon icon={faSmoking} /></span>
                <span className={classes.omiseIcon}><FontAwesomeIcon icon={faSmokingBan} /></span>
              </p>
            </Grid>
          </Grid>
        </Paper>
      </Grid>
      <Grid item xs={12} className={classes.subhead}>
        <Typography variant="h3">お店について</Typography>
      </Grid>
      <Grid item xs={12}>
        <Paper variant="outlined" elevation={3} className={classes.paper}>
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>営業時間</p>
            </Grid>
            <Grid item xs={8}>
              <p>平日・土日祝　11:00 〜 20:00</p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>HP</p>
            </Grid>
            <Grid item xs={8}>
              <p><a href="https://www.comfull.co.jp/">https://www.comfull.co.jp/</a></p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>所在地</p>
            </Grid>
            <Grid item xs={8}>
              <p>東京都文京区千駄木2-33-8 TKB千駄木ビル2F-3F</p>
              <Button
                variant="contained"
                color="primary"
                href={"https://www.google.com/maps/dir/?api=1&destination=" + encodedAddress}
              >
                行き方を調べる
              </Button>
            </Grid>
          </Grid>
        </Paper>
      </Grid>
    </Grid>
  );
}

export default UserOmise;

