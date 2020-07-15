import React, { useState, useEffect } from 'react'
import { useParams } from 'react-router-dom';
import { useForm, Controller, useFieldArray} from "react-hook-form";

import {getOmise, Omise, setOmise, OmiseForm, Ima} from 'utils/api/omise';

import { makeStyles } from '@material-ui/core/styles';
import Grid from '@material-ui/core/Grid';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import { Input, Select, Checkbox, MenuItem, TextField} from "@material-ui/core";
import { FormControlLabel } from "@material-ui/core";
import Divider from '@material-ui/core/Divider';
import Paper from '@material-ui/core/Paper';

import Snackbar from "@material-ui/core/Snackbar";
import MuiAlert from "@material-ui/lab/Alert";

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
  paper: {
    padding: theme.spacing(2),
    color: theme.palette.text.secondary,
  },
  form: {
    width: "100%",
    display: "flex",
  },
  checkboxGroup: {
    margin: theme.spacing(3),
  },
  button: {
    display: "flex",
    margin: theme.spacing(2),
  },
  flex: {
    flex: 1,
  },

  linkIcon: {
    margin: '0 0.5rem',
    '& > span': {
      fontSize: "2rem",
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

const defaultValues: OmiseForm = {
  namae: "",
  ima: new Array<Ima>(
    {namae: "混み具合", status: "Hima"},
  ),
  kefuKara: 10,
  kefuMade: 18,
  hitokoto: "",
  omotenashi: new Set<string>([]),
  oshiharai: new Set<string>([]),
  yotei: "",
  oshirase: "",
  hp: "",
  twitter: "",
  facebook: "",
  instagram: "",
  line: "",
  postcode: 1001001,
  prefcode: 1,
  city: "",
  street: "",
  building: "",
};

const service = [
  "cafe",
  "smoking",
  "non-smoking",
  "restroom",
  "wifi",
  "alcohol",
  "power",
  "軽食",
  "お菓子",
  "飲み物",
  "お土産",
  "弁当",
  "フリードリンク",
  "レンタル",
  "コピー",
]
const payment = [
  "cash",
  "jcb",
  "visa",
  "master",
  "amex",
  "diners",
  "discover",
  "交通系IC",

  "applepay",
  "paypay",
  "alipay",
  "linepay",
  "iD",
  "aupay",
  "quickpay",
  "docomo",
]

function StaffOmise() {
  console.log("render")
  const {env, clientId, omiseId} = useParams<RouteParams>();

  const [token, setToken] = useState("");
  const [omotenashi, setOmotenashi] = useState(new Set<string>([]));
  const [oshiharai, setOshiharai] = useState(new Set<string>([]));
  const { register, handleSubmit, reset, control, getValues, errors } = useForm({defaultValues});
  const { fields, append, remove } = useFieldArray({
    control,
    name: "ima"
  });
  const [status, setStatus] = useState({
    open: false,
    type: "success",
    message: "成功しました。"
  });

  const load = () => {
    getOmise(env, clientId, omiseId, (omise: Omise) => {
      reset({
        namae: omise.namae,
        ima: omise.ima,
        kefuKara: omise.kefuKara.getHours(),
        kefuMade: (omise.kefuKara.getDay() === omise.kefuMade.getDay() ? omise.kefuMade.getHours() : omise.kefuMade.getHours() + 24),
        hitokoto: omise.hitokoto,
        omotenashi: omise.omotenashi,
        oshiharai: omise.oshiharai,
        yotei: omise.yotei,
        oshirase: omise.oshirase,
        hp: omise.link.hp,
        twitter: omise.link.twitter,
        facebook: omise.link.facebook,
        instagram: omise.link.instagram,
        line: omise.link.line,
        postcode: omise.otokoro.postcode,
        prefcode: omise.otokoro.prefcode,
        city: omise.otokoro.city,
        street: omise.otokoro.street,
        building: omise.otokoro.building,
      })
      setOmotenashi(omise.omotenashi)
      setOshiharai(omise.oshiharai)
    }, (err: Error) => {
      setStatus({
        open: true,
        type: "error",
        message: "お店情報取得に失敗しました"
      });
    })
  };
  useEffect(() => {
    liff.ready.then(() => {
      let accessToken = ""
      if (!liff.isLoggedIn()) {
        if (process.env.NODE_ENV === "production") {
          liff.login({redirectUri: window.location.href})
        }
      } else {
        accessToken = liff.getAccessToken()
        if (env !== "prd") {
          liff.getProfile().then(profile => {
            console.log(profile)
          })
        }
        setToken(accessToken)
      }
      load()
    })
  },[env, clientId, omiseId])


  function handleSelectOmotenashi(name: string) {
    let omotenashi = new Set<string>(getValues("omotenashi"))
     omotenashi.has(name)
      ? omotenashi.delete(name)
      : omotenashi.add(name)
    setOmotenashi(omotenashi)
    return omotenashi
  }
  function handleSelectOshiharai(name: string) {
    let oshiharai = new Set<string>(getValues("oshiharai"))
     oshiharai.has(name)
      ? oshiharai.delete(name)
      : oshiharai.add(name)
    setOshiharai(oshiharai)
    return oshiharai
  }

  const handleClose = () => {
    setStatus({ ...status, open: false });
  };

  const onSubmit = (omise: OmiseForm) => {
    setOmise(env, clientId, omiseId, omise, token, ()=>{
      setStatus({
        open: true,
        type: "success",
        message: "お店情報を更新しました"
      });
    },
      (err: Error)=> {
        setStatus({
          open: true,
          type: "error",
          message: "お店情報を更新できませんでした"
        });
      });
  };

  const classes = useStyles();
  return (
    <Grid container className={classes.root} spacing={3}>
      <Snackbar open={status.open} autoHideDuration={3000} onClose={handleClose}>
        <MuiAlert
          onClose={handleClose}
          severity={status.type === "success" ? "success" : "error"}
          elevation={6}
          variant="filled"
        >
          {status.message}
        </MuiAlert>
      </Snackbar>
      <Grid item xs={12}>
        <Paper variant="outlined" elevation={3} className={classes.paper}>
          <Typography variant="h1" className={classes.title}>お店情報編集</Typography>
        </Paper>
      </Grid>
      <form className={classes.form} onSubmit={handleSubmit(onSubmit)}>
        <Grid container className={classes.root} spacing={3}>
          <Grid item xs={12} className={classes.subhead}>
            <Typography variant="h3">今日の情報</Typography>
          </Grid>
          <Grid item xs={12}>
            <Paper variant="outlined" elevation={3} className={classes.paper}>
							{fields.map((item, i) =>
								<Grid key={item.id} container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
									<Grid item xs={4} className={classes.head}>
										<Input
											type="text"
											placeholder="区分け"
											name={`ima[${i}].namae`}
                      defaultValue={item.namae} // これ必須
											fullWidth
											inputRef={register({required: true, maxLength: 144})}
										/>
										{errors.ima && errors.ima[i]?.namae?.message}
									</Grid>
									<Grid item xs={8}>
                    <Grid container alignItems="center">
                      <Controller
                        as={
                          <Select>
                            <MenuItem value={"Wakaran"}>未設定</MenuItem>
                            <MenuItem value={"Yasumi"}>休み</MenuItem>
                            <MenuItem value={"Hima"}>空いてる</MenuItem>
                            <MenuItem value={"Bochibochi"}>ぼちぼち</MenuItem>
                            <MenuItem value={"Isogashi"}>混雑</MenuItem>
                            <MenuItem value={"Manseki"}>満席</MenuItem>
                            <MenuItem value={"Kashikiri"}>貸切</MenuItem>
                          </Select>
                        }
                        className={classes.flex}
                        control={control}
                        name={`ima[${i}].status`}
                        defaultValue={item.status} // これ必須
                      />
                      <Button className={classes.button} variant="contained" color="secondary" onClick={() => remove(i)}>削除</Button>
                    </Grid>
                    {errors.ima && errors.ima[i]?.status?.message}
									</Grid>
								</Grid>
							)
							}
              <Divider />
              <Grid container spacing={0} justify="flex-end" alignItems="center">
                混雑区分 : 
                <Button className={classes.button} variant="contained" color="primary" onClick={() => append({namae: "区分け", status:"Wakaran"})}>追加</Button>
              </Grid>

              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>営業時間</p>
                </Grid>
                <Grid item xs={8}>
                  <Controller
                    as={
                      <Select name="kara">
                        <MenuItem value={0}>0:00</MenuItem>
                        <MenuItem value={1}>1:00</MenuItem>
                        <MenuItem value={2}>2:00</MenuItem>
                        <MenuItem value={3}>3:00</MenuItem>
                        <MenuItem value={4}>4:00</MenuItem>
                        <MenuItem value={5}>5:00</MenuItem>
                        <MenuItem value={6}>6:00</MenuItem>
                        <MenuItem value={7}>7:00</MenuItem>
                        <MenuItem value={8}>8:00</MenuItem>
                        <MenuItem value={9}>9:00</MenuItem>
                        <MenuItem value={10}>10:00</MenuItem>
                        <MenuItem value={11}>11:00</MenuItem>
                        <MenuItem value={12}>12:00</MenuItem>
                        <MenuItem value={13}>13:00</MenuItem>
                        <MenuItem value={14}>14:00</MenuItem>
                        <MenuItem value={15}>15:00</MenuItem>
                        <MenuItem value={16}>16:00</MenuItem>
                        <MenuItem value={17}>17:00</MenuItem>
                        <MenuItem value={18}>18:00</MenuItem>
                        <MenuItem value={19}>19:00</MenuItem>
                        <MenuItem value={20}>20:00</MenuItem>
                      </Select>
                    }
                    control={control}
                    name="kefuKara"
                  />
                  -
                  <Controller
                    as={
                      <Select name="made" ref={register}>
                        <MenuItem value={17}>17:00</MenuItem>
                        <MenuItem value={18}>18:00</MenuItem>
                        <MenuItem value={19}>19:00</MenuItem>
                        <MenuItem value={20}>20:00</MenuItem>
                        <MenuItem value={21}>21:00</MenuItem>
                        <MenuItem value={22}>22:00</MenuItem>
                        <MenuItem value={23}>23:00</MenuItem>
                        <MenuItem value={24}>24:00</MenuItem>
                        <MenuItem value={25}>翌1:00</MenuItem>
                        <MenuItem value={26}>翌2:00</MenuItem>
                        <MenuItem value={27}>翌3:00</MenuItem>
                        <MenuItem value={28}>翌4:00</MenuItem>
                        <MenuItem value={29}>翌5:00</MenuItem>
                        <MenuItem value={30}>翌6:00</MenuItem>
                        <MenuItem value={31}>翌7:00</MenuItem>
                      </Select>
                    }
                    control={control}
                    name="kefuMade"
                  />
                  {errors.kefuKara && errors.kefuKara.message}
                  {errors.kefuMade && errors.kefuMade.message}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>一言</p>
                </Grid>
                <Grid item xs={8}>
                  <Input type="text" placeholder="お店から一言" name="hitokoto" fullWidth inputRef={register({required: true, maxLength: 144})} />
                  {errors.hitokoto && errors.hitokoto.message}
                </Grid>
              </Grid>
            </Paper>
          </Grid>
          <Grid item xs={12} className={classes.subhead}>
            <Typography variant="h3">基本情報</Typography>
          </Grid>
          <Grid item xs={12}>
            <Paper variant="outlined" elevation={3} className={classes.paper}>
              <Grid container className={classes.root} spacing={3}>
                <Grid item xs={4} className={classes.head}>
                  <p>店舗名</p>
                </Grid>
                <Grid item xs={8}>
                  <Input type="text" placeholder="店舗名" name="namae" fullWidth inputRef={register({required: "必須です", maxLength: 16})} />
                  {errors.namae && errors.namae.message}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>サービス</p>
                </Grid>
                <Grid item xs={8} className="classes.checkboxGroup">
                  {service.map(name => (
                    <FormControlLabel
                      control={
                        <Controller
                          as={<Checkbox />}
                          control={control}
                          checked={omotenashi.has(name)}
                          name="omotenashi"
                          onChange={() => handleSelectOmotenashi(name)}
                        />
                      }
                      key={name}
                      label={name}
                    />
                  ))}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>お支払い</p>
                </Grid>
                <Grid item xs={8} className="classes.checkboxGroup">
                  {payment.map(name => (
                    <FormControlLabel
                      control={
                        <Controller
                          as={<Checkbox />}
                          control={control}
                          checked={oshiharai.has(name)}
                          name="oshiharai"
                          onChange={() => handleSelectOshiharai(name)}
                        />
                      }
                      key={name}
                      label={name}
                    />
                  ))}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>営業時間</p>
                </Grid>
                <Grid item xs={8}>
                  <TextField
                    name="yotei"
                    placeholder="平日:10:00 - 23:00\n土日祝:10:00 - 23:00\n定休日: なし"
                    fullWidth
                    inputRef={register({required: true, maxLength: 1024})}
                  />
                  {errors.yotei && errors.yotei.message}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>お知らせ</p>
                </Grid>
                <Grid item xs={8}>
                  <TextField
                    name="oshirase"
                    placeholder="コロナウイルスにかかる対応について"
                    fullWidth
                    inputRef={register({required: true, maxLength: 1024})}
                  />
                  {errors.oshirase && errors.oshirase.message}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>HP・SNS</p>
                </Grid>
                <Grid item xs={8}>
                  <div>
                    <Grid container alignItems="center">
                      <p className={classes.linkIcon}>
                        <span><FontAwesomeIcon icon={["fas", "mobile-alt"]} className={classes.hp}/></span>
                      </p>
                      <Input type="hp" placeholder="ホームページのURL" name="hp" className={classes.flex} inputRef={register} />
                    </Grid>
                    {errors.hp && errors.hp.message}
                  </div>
                  <div>
                    <Grid container alignItems="center">
                      <p className={classes.linkIcon}>
                        <span><FontAwesomeIcon icon={['fab', 'twitter-square']} className={classes.twitter}/></span>
                      </p>
                      <Input type="twitter" placeholder="https://www.twitter.com/" name="twitter" className={classes.flex} inputRef={register} />
                    </Grid>
                    {errors.twitter && errors.twitter.message}
                  </div>
                  <div>
                    <Grid container alignItems="center">
                      <p className={classes.linkIcon}>
                        <span><FontAwesomeIcon icon={['fab', 'facebook-square']} className={classes.facebook}/></span>
                      </p>
                      <Input type="facebook" placeholder="https://www.facebook.com/" name="facebook" className={classes.flex} inputRef={register} />
                    </Grid>
                    {errors.facebook && errors.facebook.message}
                  </div>
                  <div>
                    <Grid container alignItems="center">
                      <p className={classes.linkIcon}>
                        <span><FontAwesomeIcon icon={['fab', 'instagram-square']} className={classes.instagram}/></span>
                      </p>
                      <Input type="instagram" placeholder="https://www.instagram.com/" name="instagram" className={classes.flex} inputRef={register} />
                    </Grid>
                    {errors.instagram && errors.instagram.message}
                  </div>
                  <div>
                    <Grid container alignItems="center">
                      <p className={classes.linkIcon}>
                        <span><FontAwesomeIcon icon={['fab', 'line']} className={classes.line} /></span>
                      </p>
                      <Input type="line" placeholder="LINEの友達追加URL" name="line" className={classes.flex} inputRef={register} />
                    </Grid>
                    {errors.line && errors.line.message}
                  </div>
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>郵便番号</p>
                </Grid>
                <Grid item xs={8}>
                  {errors.postcode && errors.postcode.message}
                  <Input
                    type="number"
                    placeholder="1530063"
                    name="postcode"
                    fullWidth
                    inputRef={register({required: true, max: 9999999, min: 1000000})}
                  />
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>都道府県</p>
                </Grid>
                <Grid item xs={8}>
                  <Controller
                    as={
                      <Select>
                        <MenuItem value={1}>北海道</MenuItem>
                        <MenuItem value={2}>青森県</MenuItem>
                        <MenuItem value={3}>岩手県</MenuItem>
                        <MenuItem value={4}>宮城県</MenuItem>
                        <MenuItem value={5}>秋田県</MenuItem>
                        <MenuItem value={6}>山形県</MenuItem>
                        <MenuItem value={7}>福島県</MenuItem>
                        <MenuItem value={8}>茨城県</MenuItem>
                        <MenuItem value={9}>栃木県</MenuItem>
                        <MenuItem value={10}>群馬県</MenuItem>
                        <MenuItem value={11}>埼玉県</MenuItem>
                        <MenuItem value={12}>千葉県</MenuItem>
                        <MenuItem value={13}>東京都</MenuItem>
                        <MenuItem value={14}>神奈川県</MenuItem>
                        <MenuItem value={15}>新潟県</MenuItem>
                        <MenuItem value={16}>富山県</MenuItem>
                        <MenuItem value={17}>石川県</MenuItem>
                        <MenuItem value={18}>福井県</MenuItem>
                        <MenuItem value={19}>山梨県</MenuItem>
                        <MenuItem value={20}>長野県</MenuItem>
                        <MenuItem value={21}>岐阜県</MenuItem>
                        <MenuItem value={22}>静岡県</MenuItem>
                        <MenuItem value={23}>愛知県</MenuItem>
                        <MenuItem value={24}>三重県</MenuItem>
                        <MenuItem value={25}>滋賀県</MenuItem>
                        <MenuItem value={26}>京都府</MenuItem>
                        <MenuItem value={27}>大阪府</MenuItem>
                        <MenuItem value={28}>兵庫県</MenuItem>
                        <MenuItem value={29}>奈良県</MenuItem>
                        <MenuItem value={30}>和歌山県</MenuItem>
                        <MenuItem value={31}>鳥取県</MenuItem>
                        <MenuItem value={32}>島根県</MenuItem>
                        <MenuItem value={33}>岡山県</MenuItem>
                        <MenuItem value={34}>広島県</MenuItem>
                        <MenuItem value={35}>山口県</MenuItem>
                        <MenuItem value={36}>徳島県</MenuItem>
                        <MenuItem value={37}>香川県</MenuItem>
                        <MenuItem value={38}>愛媛県</MenuItem>
                        <MenuItem value={39}>高知県</MenuItem>
                        <MenuItem value={40}>福岡県</MenuItem>
                        <MenuItem value={41}>佐賀県</MenuItem>
                        <MenuItem value={42}>長崎県</MenuItem>
                        <MenuItem value={43}>熊本県</MenuItem>
                        <MenuItem value={44}>大分県</MenuItem>
                        <MenuItem value={45}>宮崎県</MenuItem>
                        <MenuItem value={46}>鹿児島県</MenuItem>
                        <MenuItem value={47}>沖縄県</MenuItem>
                      </Select>
                    }
                    control={control}
                    name="prefcode"
                    fullWidth
                  />
                  {errors.prefcode && errors.prefcode.message}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>市区町村</p>
                </Grid>
                <Grid item xs={8}>
                  <Input type="text" placeholder="目黒区" name="city" fullWidth inputRef={register({required: true, maxLength: 16})} />
                  {errors.city && errors.city.message}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>丁目・番地</p>
                </Grid>
                <Grid item xs={8}>
                  <Input type="text" placeholder="目黒2-11-3" name="street" fullWidth inputRef={register({required: true, maxLength: 32})} />
                  {errors.street && errors.street.message}
                </Grid>
              </Grid>
              <Divider />
              <Grid container className={classes.root} spacing={0} justify="flex-start" alignItems="center">
                <Grid item xs={4} className={classes.head}>
                  <p>建物</p>
                </Grid>
                <Grid item xs={8}>
                  <Input type="text" placeholder="印刷工場1F" name="building" fullWidth inputRef={register} />
                  {errors.building && errors.building.message}
                </Grid>
              </Grid>
              <Divider />
              <Grid container spacing={0} justify="flex-end" alignItems="center">
                <Button className={classes.button} variant="contained" color="secondary" onClick={load}>取消</Button>
                <Button className={classes.button} variant="contained" color="primary" type="submit">更新する</Button>
              </Grid>
            </Paper>
          </Grid>
        </Grid>
      </form>
    </Grid>
  );
}

export default StaffOmise;
