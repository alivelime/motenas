import React from 'react';
import {
  BrowserRouter as Router,
  Route,
  Switch,
} from 'react-router-dom';

import { MuiThemeProvider, createMuiTheme } from '@material-ui/core/styles';
import CssBaseline from "@material-ui/core/CssBaseline";

import Grid from '@material-ui/core/Grid';
import Paper from '@material-ui/core/Paper';

import UserOmise from './UserOmise';
// import UserOrder from './UserOrder';

import StaffOmise from './StaffOmise';

const theme = createMuiTheme({
  palette: {
    primary: {
      // light: will be calculated from palette.primary.main,
      main: '#AAD378',
      // dark: will be calculated from palette.primary.main,
      // contrastText: will be calculated to contrast with palette.primary.main
    },
    secondary: {
      main: '#FDD9D9',
    },
  },
  typography: {
    fontFamily: [
      '"M PLUS 1p"',
      'sans-serif',
    ].join(','),
    fontSize: 16,
    h1: {
      fontSize: "1.75rem"
    },
    h2: {
      fontSize: "1.5rem"
    },
    h3: {
      fontSize: "1.25rem"
    },
    h4: {
      fontSize: "1.125rem"
    },
    h5: {
      fontSize: "1rem"
    },
    h6: {
      fontSize: "1rem"
    },
  }
});


function App() {
  liff.init({ liffId: process.env.REACT_APP_LIFF_ID as string }).then(() => {})
  return (
    <MuiThemeProvider theme={theme}>
      <CssBaseline />
      <Router>
        <Switch>
          <Route path="/" exact>
            This is minarai chan liff.
          </Route>
          <Route path="/:env/user/omise/:clientId/:omiseId/:charaId" exact>
            <UserOmise />
          </Route>
          <Route path="/:env/user/order/:clientId/:omiseId/:charaId" exact>
            <Grid container alignItems="center" justify="center">
              <Grid item xs={8}>
                <Paper>7月公開予定</Paper>
              </Grid>   
            </Grid>
          </Route>

          <Route path="/:env/staff/omise/:clientId/:omiseId/:charaId" exact>
            <StaffOmise />
          </Route>
        </Switch>
      </Router>
    </MuiThemeProvider >
  );
}

export default App;
