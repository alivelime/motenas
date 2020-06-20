import React, { useState, useEffect } from 'react'
import {
  useParams,
} from 'react-router-dom';

import {getOmise, checkOmise, newOmise } from 'utils/api/omise';

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
  faWifi,
  faBeer,
  faPlug,
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
    '& > span': {
      fontSize: "2rem",
      margin: '0 0.2rem',
    }
  },
  omiseOmotenashi: {
    '& > span': {
      fontSize: "1rem",
      margin: '0 0.2rem',
    }
  },
}));


function UserOmise() {
  const {env, clientId, omiseId, charaId} = useParams<RouteParams>();
  const [omise, setOmise] = useState(newOmise());

  useEffect(() => {
    liff.ready.then(() => {
      let accessToken = ""
      if (!liff.isLoggedIn()) {
        // liff.login({})
      } else {
        accessToken = liff.getAccessToken()
      }
      getOmise(env, clientId, omiseId, (omise) => setOmise(omise))
      checkOmise(env, clientId, omiseId, charaId, accessToken);
    })
  },[env, clientId, omiseId, charaId])


  const classes = useStyles();
  return (
    <Grid container className={classes.root} spacing={3}>
      <Grid item xs={12} justify="center">
        <Paper variant="outlined" elevation={3} className={classes.paper}>
          <Typography variant="h1" className={classes.title}>{omise.namae}</Typography>
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
              <p>{omise.ima === 1 ? "本日休業" : omise.kefuKara.getHours() +":00 〜 "+omise.kefuMade.getHours()+":00"}</p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>混み具合</p>
            </Grid>
            <Grid item xs={8}>
              <p>{(() => {
                switch (omise.ima) {
                  case 0: return "未設定です";
                  case 1: return "お休みです";
                  case 2: return "快適です";
                  case 3: return "賑わっています";
                  case 4: return "大盛況です";
                  case 5: return "貸切です";
                }
              })()}
              </p>
            </Grid>
          </Grid>
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>一言</p>
            </Grid>
            <Grid item xs={8}>
              <p>{omise.hitokoto}</p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>その他</p>
            </Grid>
            <Grid item xs={8}>
              <p className={classes.omiseIcon}>
              {omise.omotenashi.map(s => {switch (s) {
                case "cafe": return <span><FontAwesomeIcon icon={faCoffee} /></span>;
                case "smoking": return <span><FontAwesomeIcon icon={faSmoking} /></span>;
                case "non-smoking": return <span><FontAwesomeIcon icon={faSmokingBan} /></span>;
                case "restroom": return <span><FontAwesomeIcon icon={faRestroom} /></span>;
                case "wifi": return <span><FontAwesomeIcon icon={faWifi} /></span>;
                case "alcohol": return <span><FontAwesomeIcon icon={faBeer} /></span>;
                case "power": return <span><FontAwesomeIcon icon={faPlug} /></span>;
                default: return null;
              }})}
              </p>
              <p className={classes.omiseOmotenashi}>
              {omise.omotenashi.map(s => {switch (s) {
                case "cafe": return null;
                case "smoking": return null;
                case "non-smoking": return null;
                case "restroom": return null;
                case "wifi": return null;
                case "alcohol": return null;
                case "power": return null;
                default: return <span>{s}</span>;
              }})}
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
              <p>{omise.yotei}</p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>HP</p>
            </Grid>
            <Grid item xs={8}>
              <p><a href="{omise.url}">{omise.url}</a></p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>所在地</p>
            </Grid>
            <Grid item xs={8}>
              <p>{omise.otokoro.forMap}{omise.otokoro.building}</p>
              <p>{omise.otokoro.access}</p>
              <Button
                variant="contained"
                color="primary"
                href={"https://www.google.com/maps/dir/?api=1&destination=" + encodeURIComponent(omise.otokoro.forMap)}
                target="_blank"
                rel="noreferrer noopener"
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

