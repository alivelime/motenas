import React, { useRef, useEffect } from 'react'
import {
  // BrowserQRCodeReader, こっちは使わない
  QRCodeReader,
  NotFoundException,
  ChecksumException,
  FormatException,
  HTMLCanvasElementLuminanceSource,
  HybridBinarizer,
  BinaryBitmap,
} from '@zxing/library';

import { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
    width: '100%',
  },
  paper: {
    padding: theme.spacing(2),
    color: theme.palette.text.secondary,
    textAlign: 'center',
  },
  canvasStyle: {
    top: '5%',
    left: 0,
    position: 'absolute',
    overflow: 'hidden',
    display: debug => debug ? 'block' : 'none',
  },
  previewStyle: {
    top: '5%',
    left: 0,
    position: 'absolute',
    overflow: 'hidden',
    width: debug => debug ? '1px' : '100%',
    height: debug => debug ? '1px' : '95%',
    display: 'block',
  },
  viewFinderStyle: {
    top: '5%',
    left: 0,
    zIndex: 1,
    boxSizing: 'border-box',
    borderLeft: '90px solid rgba(0, 0, 0, 0.3)',
    borderRight: '90px solid rgba(0, 0, 0, 0.3)',
    borderTop: '160px solid rgba(0, 0, 0, 0.3)',
    borderBottom: '160px solid rgba(0, 0, 0, 0.3)',
    boxShadow: 'inset 0 0 0 5px rgba(255, 0, 0, 0.5)',
    position: 'absolute',
    width: '100%',
    height: '95%',
    display: debug => debug ? 'none' : 'block',
  }
}));

interface Props {
  constraints: MediaTrackConstraints,
  delay: number,
  resolution: number,
  onScan: (text: string) => void;
  onError: (err: Error) => void;
  debug: boolean,
}
function QrReader(props: Props) {
  const classes = useStyles(props.debug);
  const canvasElement = useRef<HTMLCanvasElement>(null);
  const previewElement = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    let videoStream: MediaStream | null = null
    
    // workerに実装したいけど、gulp作るの面倒だし、600x600pxなら100msかからないし、全画面これだからまあいいか。
    const check = () => {
      try {
        const canvas = canvasElement.current;
        const preview = previewElement.current;
        if (!canvas || !preview) {
          return setTimeout(check, props.delay)
        }

        // videoからcanvasに切り出し
        if (preview && preview.readyState === preview.HAVE_ENOUGH_DATA) {
          canvas.width = props.resolution
          canvas.height = props.resolution

          // 真ん中600px切り出し
          const ctx = canvas.getContext('2d')
          if (!ctx) {
            return setTimeout(check, props.delay)
          }
          ctx.drawImage(preview, 
            (preview.videoWidth - props.resolution) / 2,
            (preview.videoHeight - props.resolution) / 2,
            props.resolution,
            props.resolution,
            0, 0, props.resolution, props.resolution
          )

          const codeReader = new QRCodeReader()
          const luminanceSource = new HTMLCanvasElementLuminanceSource(canvas);
          const hybridBinarizer = new HybridBinarizer(luminanceSource);
          const binaryBitmap = new BinaryBitmap(hybridBinarizer);
          const result = codeReader.decode(binaryBitmap)

          props.onScan(result.getText());
        }

        setTimeout(check, props.delay)
      } catch (e) {
        const isNotFound = e instanceof NotFoundException;
        const isChecksumOrFormatError = e instanceof ChecksumException || e instanceof FormatException;
        if (isChecksumOrFormatError || isNotFound) {
          // trying again
          return setTimeout(check, props.delay)
        }

        props.onError(e)
      }
    }
    const handleLoadStart = () => {
      const preview = previewElement.current;
      if (preview) {
        preview.play()

        // Some browsers call loadstart continuously
        preview.removeEventListener('loadstart', handleLoadStart)

        setTimeout(check, props.delay);
      }
    }
    const handleVideo = (stream: MediaStream) => {
      // 準備ができるまで待つ
      if (!previewElement || !previewElement.current) {
        return setTimeout(handleVideo, 200, stream)
      }
      const preview = previewElement.current;

      // Handle different browser implementations of MediaStreams as src
      if((preview || {}).srcObject !== undefined){
        preview.srcObject = stream
      // @ts-ignore
      } else if (preview.mozSrcObject !== undefined) {
        // @ts-ignore
        preview.mozSrcObject = stream
      } else if (window.URL.createObjectURL) {
        preview.src = window.URL.createObjectURL(stream)
      } else if (window.webkitURL) {
        preview.src = window.webkitURL.createObjectURL(stream)
      } else {
        // @ts-ignore
        preview.src = stream
      }
      //
      // IOS play in fullscreen
      // @ts-ignore
      preview.setAttribute('autoplay', 'true');
      preview.setAttribute('muted', 'true');
      preview.setAttribute('playsinline', 'true');

      videoStream = stream
      preview.addEventListener('loadstart', handleLoadStart)
    }
    const initVideo = () => {
      Promise.resolve(props.constraints)
        .then(video => navigator.mediaDevices.getUserMedia({ video }))
        .then(handleVideo)
        .catch(props.onError)
    };

    initVideo() 

    return () => {
      if (videoStream) {
        videoStream.getVideoTracks().forEach(t => t.stop())
        videoStream = null
      }
    }
  })

  return (
    <div className={classes.root}>
      <div className={classes.viewFinderStyle}></div>
      <canvas className={classes.canvasStyle} ref={canvasElement} ></canvas>
      <video
        ref={previewElement}
        className={classes.previewStyle}
      >
      </video>
    </div>
  );
}

export default QrReader;
