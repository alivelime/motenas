import fetcher from 'utils/fetcher'


interface ResponseOmise {
  omise: Omise;
}

export interface Omise {
  clientId: string;
  omiseId: string;
  namae: string;
  link: Links;
  yotei: string;
  oshirase: string;
  omotenashi: Set<string>;
  oshiharai: Set<string>;
  otokoro: Address;

  ima: Array<Ima>,
  hitokoto: "",
  kefuKara: Date;
  kefuMade: Date;
  createdAt: Date;
  updatedAt: Date;

  isYasumi(): boolean;
}
export interface Links {
  hp: string,
  twitter: string,
  facebook: string,
  instagram: string,
  line: string,
}

// 単に export interface Ima {...} とすると、
// Attempted import error: 'Ima' is not exported from 'utils/api/omise'.
// とか言われる。というか今はエラーにならない。。意味わからん。。
export interface Ima {
  namae: string,
  status: string,
}

export interface Address {
  country: string;
  postcode: number;
  forMap: string;
  todofuken: string;
  prefcode: number;
  city: string;
  street: string;
  building: string;
  access: string;
}

export interface OmiseForm {
  namae: string,
  ima: Array<Ima>,
  hitokoto: string,
  kefuKara: number,
  kefuMade: number,
  omotenashi: Set<string>,
  oshiharai: Set<string>,
  yotei: string,
  oshirase: string,
  hp: string,
  twitter: string,
  facebook: string,
  instagram: string,
  line: string,
  postcode: number,
  prefcode: number,
  city: string,
  street: string,
  building: string,
}

export function newOmise(): Omise {
  return {
    clientId: "",
    omiseId: "",
    namae: "",
    link: {
      hp: "",
      twitter: "",
      facebook: "",
      instagram: "",
      line: "",
    },
    yotei: "",
    oshirase: "",
    omotenashi: new Set<string>([]),
    oshiharai: new Set<string>([]),
    otokoro: {
      country: "",
      postcode: 1000001,
      forMap: "",
      todofuken: "",
      prefcode: 1,
      city: "",
      street: "",
      building: "",
      access: "",
    },
    ima: new Array<Ima>(),
    hitokoto: "",
    kefuKara: new Date(),
    kefuMade: new Date(),
    createdAt: new Date(),
    updatedAt: new Date(),
    isYasumi: function() {
      return this.ima.every(ima => ima.status === "Yasumi")
    },
  };
}

export function getOmise(
  env: string,
  clientId: string,
  omiseId: string,
  resolve: (omise: Omise)=>void,
  reject: (err: Error)=>void,
) {
  fetcher<ResponseOmise>(
    env === 'prd'
      ? `${process.env.REACT_APP_PRD_API_HOST}/omise/${clientId}/${omiseId}`
      : `${process.env.REACT_APP_DEV_API_HOST}/omise/${clientId}/${omiseId}`,
    {
      method: "GET",
      mode: "cors",
      cache: "no-cache",
    }
  )
    .then(res => {
      res.omise.omotenashi = new Set(res.omise.omotenashi)
      res.omise.oshiharai = new Set(res.omise.oshiharai)
      res.omise.isYasumi = newOmise().isYasumi
      resolve(res.omise)
    })
    .catch(err => reject(err))
}

export function setOmise(
  env: string,
  clientId: string,
  omiseId: string,
  charaId: string,
  omise: OmiseForm,
  token: string,
  resolve: ()=>void,
  reject: (err: Error
)=>void) {
  fetcher<void>(
    env === 'prd'
      ? `${process.env.REACT_APP_PRD_LINE_API_HOST}/line-api/omise/set`
      : `${process.env.REACT_APP_DEV_LINE_API_HOST}/line-api/omise/set`,
    {
      method: "POST",
      mode: "cors",
      cache: "no-cache",
      headers: { 'Authorization': 'Bearer: '+token},
      body: JSON.stringify({
        ...omise,
        charaUri: `${clientId}/${omiseId}/${charaId}`,
        postcode: Number(omise.postcode),
        omotenashi: Array.from(omise.omotenashi),
        oshiharai: Array.from(omise.oshiharai),
      }),
    }
  )
    .then(()=>resolve())
    .catch(err => reject(err))
}

export function checkOmise(env: string, clientId: string, omiseId: string, charaId: string, accessToken: string) {
  fetcher<void>(
    env === 'prd'
      ? `${process.env.REACT_APP_PRD_LINE_API_HOST}/line-api/omise/check`
      : `${process.env.REACT_APP_DEV_LINE_API_HOST}/line-api/omise/check`,
    {
      method: "POST",
      mode: "cors",
      cache: "no-cache",
      body: JSON.stringify({
        accessToken: accessToken,
        charaId: `${clientId}/${omiseId}/${charaId}`,
      }),
    }
  )
    .then()
    .catch(err => console.log(err))
}
