const Header: string = "JAHISTC"

export interface Okusuri {
  version: string,
  text: string,
  count: number,
  data: Array<OkusuriData>,

  add(text: string): void,
  isEnd: boolean,
  print(): string,
  getMessage(): string,
}
export interface OkusuriData {
  num: number,
  user_name: string,
  user_sex: number,
  user_birthday: string,
  
  okusuriName: string,
}

export function newOkusuri(f: boolean): Okusuri {
  if (f) {
    const speach = new SpeechSynthesisUtterance("一番左のQRにかざしてください");
    speechSynthesis.speak(speach);
  }

  return {
    version: "",
    text: "",
    count: 0,
    data: Array<OkusuriData>(),
    isEnd: false,

    add: function(text: string) {

      if (text.startsWith(Header)) {
        if (this.count > 0) {
          return
        }

        const l = text.split("\r\n")
        const [version, _] = l[0].split(',')
        this.version = version.replace(Header, '')

        this.text += l.slice(1).join("\r\n")
      } else {
        if (this.count === 0) {
          return
        }
        if (this.text.includes(text)) {
          return
        }
        this.text += text
      }

      if (text.endsWith("\r\n")) {
        /*
        this.data = this.text.split("\r\n").map(l => {
          const c = l.split(',')
          return OkusuriData {
          }
        })
         */
        this.isEnd = true
      }
      this.count++

      const speach = new SpeechSynthesisUtterance(this.getMessage())
      speechSynthesis.speak(speach)
    },
    print: function(): string {
      return this.text === ""
        ? ""
        : ` JAHISTC${this.version}` + this.text
    },
    getMessage: function(): string {
       if (this.text === "") {
        return "一番左のQRにかざしてください"
      } else if (this.isEnd) {
        return "全て読み取りました"
      } else {
        return `次のQRにかざしてください。読込QR ${this.count}つ`
      }
    },
  }
}

