import React, { useState } from 'react'

import QrReader from 'components/QrReader'

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
}));


function UserOrder() {
  const [qr, setQr] = useState(false);
  const [qrDebug, setQrDebug] = useState(false);
  const [qrText, setQrText] = useState<string | null>("");

  const classes = useStyles();
  return (
    <Grid container className={classes.root} spacing={3} justify="center" alignItems="center">
      <Grid item xs={12}>
        <Paper variant="outlined" elevation={3} className={classes.paper}>
          {qr &&
            <QrReader
              debug={false}
              delay={100}
              resolution={600}
              onError={(err)=>{alert(err); setQr(false)}}
              onScan={(text) => {if (text != null) {setQr(false); setQrText(text);}}}
              constraints={{
                width: {ideal: 1920},
                  height: {ideal: 1080},
                  facingMode: {ideal: 'environment'},
              }}
            />
          }
          {qrDebug &&
            <QrReader
              debug={true}
              delay={100}
              resolution={600}
              onError={(err)=>{alert(err); setQr(false); setQrDebug(false)}}
              onScan={(text) => {if (text != null) {setQr(false); setQrDebug(false); setQrText(text);}}}
              constraints={{
                width: {ideal: 1920},
                height: {ideal: 1080},
                facingMode: {ideal: 'environment'},
              }}
            />
          }
          {!qr && !qrDebug &&
              <p><Button variant="contained" color="primary" onClick={()=>{setQr(true); setQrDebug(false)}}>
                QRコードを読み込む
            </Button></p>
          }
          {!qr && !qrDebug &&
              <p><Button variant="contained" color="secondary" onClick={()=>{setQr(false); setQrDebug(true)}}>
                デバッグ用
            </Button></p>
          }
          <p>{qrText}</p>
        </Paper>
      </Grid>
    </Grid>
  );
}

export default UserOrder;

