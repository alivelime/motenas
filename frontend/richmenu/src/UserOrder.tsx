import React, { useState, memo, useMemo, useCallback, useEffect } from 'react'
import { useParams } from 'react-router-dom';
import liff from '@line/liff';

import QrReader from 'components/QrReader'
import { newOkusuri } from 'utils/okusuri'
import { enterOmise } from 'utils/api/denpyo'
import { callbackUserUrl, cloudfrontUrl, liffUrl } from 'utils/callback'
import { isLocal } from 'utils/env'

import { makeStyles } from '@material-ui/core/styles';
import Grid from '@material-ui/core/Grid';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import Paper from '@material-ui/core/Paper';
import Divider from '@material-ui/core/Divider';


interface RouteParams {
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
    textAlign: 'center',
  },
  message: {
    top: 0,
    left: 0,
    position: 'absolute',
    width: '100%',
    display: 'block',
  },
  title: {
    textAlign: 'center',
    fontWeight: 'bold',
  },
}));

const QrReaderMemo = memo(QrReader)

function UserOrder() {
  const classes = useStyles();
  const {clientId, omiseId} = useParams<RouteParams>();

  const [isInClient, setIsInClient] = useState(false);
  const [qr, setQr] = useState(false);
  const [qrDebug, setQrDebug] = useState(false);
  const [okusuri, setOkusuri] = useState(newOkusuri(false));

  useEffect(() => {
    liff.ready.then(() => {
      if (liff.isInClient()) {
        // LINE内ブラウザでは使えないので外部ブラウザで開いてもらう
        setIsInClient(true)
      } else {
        // 外部ブラウザ
        setIsInClient(false)
        if (!liff.isLoggedIn() && !isLocal()) {
          liff.login({redirectUri: callbackUserUrl('order', clientId, omiseId)})
        }
      }
    })
  },[clientId, omiseId])

  const constraints = useMemo(() => ({
    width: {ideal: 1920},
    height: {ideal: 1080},
    facingMode: {ideal: 'environment'},
  }), [])
  const onScan = useCallback((text: string | null) => {
    if (text != null) {
      setOkusuri(okusuri => {
        let o = {...okusuri}
        o.add(text)

        if (o.isEnd) {
          setQr(false)
          setQrDebug(false)

          const token = liff.getAccessToken()
          enterOmise(clientId, omiseId, o.print(), token,
            ()=>{
              alert("サーバに保存しました")
            },
            (err: Error)=> {
              alert("サーバに保存できませんでした" + err.toString())
            });
        }
        return o
      })
    }
  }, [clientId, omiseId])
  const onError = useCallback((err: Error) => {
    alert(err)
    setQr(false)
    setQrDebug(false)

    setOkusuri(newOkusuri(false))
  }, [])
  return (
    <Grid container className={classes.root} spacing={3} justify="center" alignItems="center">
      <Grid item xs={12}>
        {isInClient
        ? <Paper variant="outlined" elevation={3} className={classes.paper}>
          <Divider />
          <Typography variant="h3" className={classes.title}>QRコード読み取り</Typography>
          <p>セキュリティ保護の為<br />外部ブラウザを起動します</p>
          <p><Button
              variant="contained"
              color="primary"
              onClick={() => {
                liff.openWindow({
                  url: cloudfrontUrl(),
                  external: true,
                })
                liff.closeWindow()
              }}
            >
              ブラウザで開く
          </Button></p>
        </Paper>
        : okusuri.isEnd
          ? <Paper variant="outlined" elevation={3} className={classes.paper}>
            <p>お薬情報を登録しました。</p>
            <p><Button
                variant="contained"
                color="primary"
                href={liffUrl()}
              >メニューへ</Button></p>
          </Paper>
          : <Paper variant="outlined" elevation={3} className={classes.paper}>
            {(qr || qrDebug) &&
              <div className={classes.message}>{okusuri.getMessage()}</div>
            }
            {!qr && !qrDebug &&
            <p><Button
                variant="contained"
                color="primary"
                onClick={()=>{
                  setQr(true)
                  setQrDebug(false)
                  setOkusuri(newOkusuri(true))
                }}>
                  QRコードを読み込む
              </Button></p>
            }
            {!qr && !qrDebug &&
            <p><Button
                variant="contained"
                color="secondary"
                onClick={()=>{
                  setQr(false)
                  setQrDebug(true)
                  setOkusuri(newOkusuri(true))
                }}>
                  デバッグ用
              </Button></p>
            }
            <p>{okusuri.print()}</p>
            {qr &&
              <QrReaderMemo
                debug={false}
                delay={100}
                resolution={600}
                onError={onError}
                onScan={onScan}
                constraints={constraints}
              />
            }
            {qrDebug &&
              <QrReaderMemo
                debug={true}
                delay={100}
                resolution={600}
                onError={onError}
                onScan={onScan}
                constraints={constraints}
              />
            }
          </Paper>
        }
      </Grid>
    </Grid>
  );
}

export default UserOrder;

