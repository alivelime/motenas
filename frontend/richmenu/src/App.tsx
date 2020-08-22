import React from 'react';
import {
  BrowserRouter as Router,
  Route,
  Redirect,
  Switch,
} from 'react-router-dom';
import liff from '@line/liff';
import queryString from 'query-string';

import { MuiThemeProvider, createMuiTheme } from '@material-ui/core/styles';
import CssBaseline from "@material-ui/core/CssBaseline";

import { LiffId } from './utils/env';

import UserOmise from './UserOmise';
import UserOrder from './UserOrder';

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
  liff.init({ liffId: LiffId()}).then(() => {})
  return (
    <MuiThemeProvider theme={theme}>
      <CssBaseline />
      <Router>
        <Switch>
          <Route path="/" exact>
            This is minarai chan liff.
          </Route>
          <Route path="/callback" exact render={(props) => {
            const qs = queryString.parse(props.location.search)
            console.log(`/${qs.target}/${qs.page}/${qs.clientId}/${qs.omiseId}${window.location.search}`)
            return <Redirect to={`/${qs.target}/${qs.page}/${qs.clientId}/${qs.omiseId}${window.location.search}`} />
          }} />

          <Route path="/user/omise/:clientId/:omiseId" exact>
            <UserOmise />
          </Route>
          <Route path="/user/order/:clientId/:omiseId" exact>
            <UserOrder/>
          </Route>

          <Route path="/staff/omise/:clientId/:omiseId">
            <StaffOmise />
          </Route>
        </Switch>
      </Router>
    </MuiThemeProvider >
  );
}

export default App;
