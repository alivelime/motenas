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

import { library } from '@fortawesome/fontawesome-svg-core';
import { fab } from '@fortawesome/free-brands-svg-icons';
import { fas } from '@fortawesome/free-solid-svg-icons';
import { far } from '@fortawesome/free-regular-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';;;;


library.add(fab, fas, far);


interface RouteParams {
    env: string,
    clientId: string,
    omiseId: string,
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
  linkIcon: {
    '& > a': {
      fontSize: "2rem",
      margin: '0 0.2rem',
    }
  },
  hp: {
    color: theme.palette.text.primary,
  },
  twitter: {
    color: "#1da1f2",
  },
  facebook: {
    color: "#4267b2",
  },
  instagram: {
    color: "#cf2e92",
  },
  line: {
    color: "#00B900",
  },
}));


function UserOmise() {
  const {env, clientId, omiseId} = useParams<RouteParams>();
  const [omise, setOmise] = useState(newOmise());

  useEffect(() => {
    liff.ready.then(() => {
      let accessToken = ""
      if (!liff.isLoggedIn()) {
        if (process.env.NODE_ENV === "production") {
          // liff.login({})
        }
      } else {
        accessToken = liff.getAccessToken()
      }
      getOmise(env, clientId, omiseId, (omise) => setOmise(omise),(err: Error)=>{console.log(err)})
      if (process.env.NODE_ENV === "production") {
        checkOmise(env, clientId, omiseId, accessToken);
      }
    })
  },[env, clientId, omiseId])

  const classes = useStyles();
  return (
    <Grid container className={classes.root} spacing={3}>
      <Grid item xs={12}>
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
              <p>{omise.isYasumi()
                ? "本日休業"
                : omise.kefuKara.getHours() +":00 〜 "
                +(omise.kefuKara.getDay() === omise.kefuMade.getDay() ? '' : '翌')
                +omise.kefuMade.getHours()+":00"}</p>
            </Grid>
          </Grid>
          <Divider />
          {
            omise.ima.map(ima =>
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>{ima.namae || "混み具合"}</p>
                </Grid>
                <Grid item xs={8}>
                  <p>
                    {(() => {
                      switch (ima.status) {
                        case "Wakaran": return "未設定です";
                        case "Yasumi": return "お休みです";
                        case "Hima": return "空いています";
                        case "Bochibochi": return "普通です";
                        case "Isogashi": return "賑わっています";
                        case "Manseki": return "満席です";
                        case "Kashikiri": return "貸切です";
                      }
                    })()}
                  </p>
                </Grid>
              </Grid>
              )
          }
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>今日の一言</p>
            </Grid>
            <Grid item xs={8}>
              <p>{omise.hitokoto}</p>
            </Grid>
          </Grid>
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>更新日時</p>
            </Grid>
            <Grid item xs={8}>
              <p>{omise.updatedAt.toLocaleString("ja-JP")}</p>
            </Grid>
          </Grid>
          <Divider />
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
              <p>お知らせ</p>
            </Grid>
            <Grid item xs={8}>
              <p>{omise.oshirase}</p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>サービス</p>
            </Grid>
            <Grid item xs={8}>
              <p className={classes.omiseIcon}>
              {Array.from(omise.omotenashi).map(s => {switch (s) {
                case "cafe": return <span><FontAwesomeIcon icon={["fas", "coffee"]} /></span>;
                case "smoking": return <span><FontAwesomeIcon icon={["fas", "smoking"]} /></span>;
                case "non-smoking": return <span><FontAwesomeIcon icon={["fas", "smoking-ban"]} /></span>;
                case "restroom": return <span><FontAwesomeIcon icon={["fas", "restroom"]} /></span>;
                case "wifi": return <span><FontAwesomeIcon icon={["fas", "wifi"]} /></span>;
                case "alcohol": return <span><FontAwesomeIcon icon={["fas", "beer"]} /></span>;
                case "plug": return <span><FontAwesomeIcon icon={["fas", "plug"]} /></span>;
                default: return null;
              }})}
              </p>
              <p className={classes.omiseOmotenashi}>
              {Array.from(omise.omotenashi).map(s => {switch (s) {
                case "cafe": return null;
                case "smoking": return null;
                case "non-smoking": return null;
                case "restroom": return null;
                case "wifi": return null;
                case "alcohol": return null;
                case "plug": return null;
                default: return <span>{s}</span>;
              }})}
              </p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>決済方法</p>
            </Grid>
            <Grid item xs={8}>
              <p className={classes.omiseIcon}>
              {Array.from(omise.oshiharai).map(s => {switch (s) {
                case "cash": return <span><FontAwesomeIcon icon={["fas", "yen-sign"]} /></span>;
                case "visa": return <span><FontAwesomeIcon icon={['fab', 'cc-visa']} /></span>;
                case "master": return <span><FontAwesomeIcon icon={['fab', 'cc-mastercard']} /></span>;
                case "jcb": return <span><FontAwesomeIcon icon={['fab', 'cc-jcb']} /></span>;
                case "amex": return <span><FontAwesomeIcon icon={['fab', 'cc-amex']} /></span>;
                case "diners": return <span><FontAwesomeIcon icon={['fab', 'cc-diners-club']} /></span>;
                case "discover": return <span><FontAwesomeIcon icon={['fab', 'cc-discover']} /></span>;
                case "applepay": return <span><FontAwesomeIcon icon={['fab', 'apple-pay']} /></span>;
                case "alipay": return <span><FontAwesomeIcon icon={['fab', 'alipay']} /></span>;
                default: return null;
              }})}
              </p>
              <p className={classes.omiseOmotenashi}>
              {Array.from(omise.oshiharai).map(s => {switch (s) {
                case "cash": return null
                case "visa": return null
                case "master": return null
                case "jcb": return null
                case "amex": return null
                case "diners": return null
                case "discover": return null
                case "applepay": return null
                case "alipay": return null
                default: return <span>{s}</span>;
              }})}
              </p>
            </Grid>
          </Grid>
          <Divider />
          <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
            <Grid item xs={4} className={classes.head}>
              <p>リンク</p>
            </Grid>
            <Grid item xs={8}>
              <p className={classes.linkIcon}>
                {omise.link.hp &&
                <a target="_blank" href={omise.link.hp} rel="noreferrer noopener">
                  <FontAwesomeIcon icon={["fas", "mobile-alt"]} className={classes.hp}/>
                </a>}
                {omise.link.twitter &&
                <a target="_blank" href={omise.link.twitter} rel="noreferrer noopener">
                  <FontAwesomeIcon icon={['fab', 'twitter-square']} className={classes.twitter}/>
                </a>}
                {omise.link.facebook &&
                <a target="_blank" href={omise.link.facebook} rel="noreferrer noopener">
                  <FontAwesomeIcon icon={['fab', 'facebook-square']} className={classes.facebook}/>
                </a>}
                {omise.link.instagram &&
                <a target="_blank" href={omise.link.instagram} rel="noreferrer noopener">
                  <FontAwesomeIcon icon={['fab', 'instagram-square']} className={classes.instagram}/>
                </a>}
                {omise.link.line &&
                <a target="_blank" href={omise.link.line} rel="noreferrer noopener">
                  <FontAwesomeIcon icon={['fab', 'line']} className={classes.line} />
                </a>}
              </p>
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
              <p>
                <Button
                  variant="contained"
                  color="primary"
                  href={"https://www.google.com/maps/dir/?api=1&destination=" + encodeURIComponent(omise.otokoro.forMap)}
                  target="_blank"
                  rel="noreferrer noopener"
                >
                  行き方を調べる
                </Button>
              </p>
              <p>
                <Button
                  variant="outlined"
                  onClick={() => {
                    if (liff.isApiAvailable('shareTargetPicker')) {
                      liff.shareTargetPicker([
                        {
                          type: "text",
                          text: `${omise.namae}\n\nお店情報${window.location.href}\n\nLINEアカウント\n${omise.link.line}`,
                        }
                      ])
                    } else {
                      alert("Lineで友達になって頂き、「お店情報」メニューから再度アクセスしてください。")
                    }
                  }}
                  className={classes.line}
                  startIcon={<FontAwesomeIcon icon={['fab', 'line']} className={classes.line} />}
              >
                友達に教える
              </Button>
            </p>
            </Grid>
          </Grid>
        </Paper>
      </Grid>
    </Grid>
  );
}

export default UserOmise;

