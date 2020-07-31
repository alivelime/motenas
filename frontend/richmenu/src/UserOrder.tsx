import React, { useState, useEffect } from 'react'
import QrReader from 'react-qr-reader'

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


function UserOmise() {
  const [qr, setQr] = useState(false);
  const [qrText, setQrText] = useState<string | null>("");

  const classes = useStyles();
  return (
    <Grid container className={classes.root} spacing={3} justify="center" alignItems="center">
      <Grid item xs={12}>
        <Paper variant="outlined" elevation={3} className={classes.paper}>
          {qr
            ? <QrReader
              delay={1000}
              resolution={1920}
              onError={(err)=>{alert(err.toString()); setQr(false)}}
              onScan={(text) => {if (text != null) {setQrText(text); setQr(false);}}}
              style={{ width: '100%' }}
            />
            : <Button variant="contained" color="primary" onClick={()=>{setQr(true)}}>
              QRコードを読み込む
            </Button>
          }
          <p>{qrText}</p>
        </Paper>
      </Grid>
    </Grid>
  );
}

export default UserOmise;

