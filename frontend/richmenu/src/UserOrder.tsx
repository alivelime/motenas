import React, { useState, memo, useMemo, useCallback } from 'react'

import QrReader from 'components/QrReader'
import { newOkusuri } from 'utils/okusuri'

import { makeStyles } from '@material-ui/core/styles';
import Grid from '@material-ui/core/Grid';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import Paper from '@material-ui/core/Paper';
import Divider from '@material-ui/core/Divider';


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
    textAlign: 'center',
  },
  message: {
    top: 0,
    left: 0,
    position: 'absolute',
    width: '100%',
    display: 'block',
  },
}));

const QrReaderMemo = memo(QrReader)

function UserOrder() {
  const classes = useStyles();

  const [qr, setQr] = useState(false);
  const [qrDebug, setQrDebug] = useState(false);
  const [okusuri, setOkusuri] = useState(newOkusuri());

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
        }
        return o
      })
    }
  }, [])
  const onError = useCallback((err: Error) => {
    alert(err)
    setQr(false)
    setQrDebug(false)

    setOkusuri(newOkusuri())
  }, [])
  return (
    <Grid container className={classes.root} spacing={3} justify="center" alignItems="center">
      <Grid item xs={12}>
        <Paper variant="outlined" elevation={3} className={classes.paper}>
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
                setOkusuri(newOkusuri())
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
                setOkusuri(newOkusuri())
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
      </Grid>
    </Grid>
  );
}

export default UserOrder;

