import { useEffect, useRef, useState } from "react";
import Hls from "hls.js";

interface HLSPlayerProps {
  url: string;
  className?: string;
  minSegments?: number;
}

export function HLSPlayer({ url, className, minSegments = 3 }: HLSPlayerProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const hlsRef = useRef<Hls | null>(null);
  const [status, setStatus] = useState<"waiting" | "playing" | "error">("waiting");
  const [errorMsg, setErrorMsg] = useState("");

  useEffect(() => {
    let cancelled = false;
    let retryTimer: ReturnType<typeof setTimeout>;

    async function checkSegments(): Promise<boolean> {
      try {
        const ctrl = new AbortController();
        const t = setTimeout(() => ctrl.abort(), 3000);
        const resp = await fetch(url, { signal: ctrl.signal });
        clearTimeout(t);
        if (!resp.ok) return false;

        const body = await resp.text();
        if (!body.includes("#EXTM3U")) return false;

        // 마스터 m3u8 → 하위 m3u8 참조 찾기
        const subMatch = body.match(/^[^#\s].+\.m3u8$/m);
        if (subMatch) {
          const subUrl = url.replace(/[^/]+$/, subMatch[0]);
          const subCtrl = new AbortController();
          const st = setTimeout(() => subCtrl.abort(), 3000);
          const subResp = await fetch(subUrl, { signal: subCtrl.signal });
          clearTimeout(st);
          const subBody = await subResp.text();
          return (subBody.match(/#EXTINF/g) || []).length >= minSegments;
        }

        return (body.match(/#EXTINF/g) || []).length >= minSegments;
      } catch {
        return false;
      }
    }

    async function run() {
      while (!cancelled) {
        // 1. 세그먼트가 충분할 때까지 폴링
        setStatus("waiting");
        let ready = false;
        for (let i = 0; i < 120 && !cancelled; i++) {
          if (await checkSegments()) { ready = true; break; }
          await new Promise((r) => { retryTimer = setTimeout(r, 1000); });
        }
        if (cancelled) return;
        if (!ready) {
          setStatus("error");
          setErrorMsg("스트림 대기 시간 초과");
          return;
        }

        // 2. hls.js로 재생 시도
        const result = await tryPlay();
        if (cancelled) return;

        // 성공이면 끝, 실패면 다시 1부터
        if (result === "playing") return;
        // 실패 시 2초 대기 후 재시도
        await new Promise((r) => { retryTimer = setTimeout(r, 2000); });
      }
    }

    function tryPlay(): Promise<"playing" | "failed"> {
      return new Promise((resolve) => {
        const video = videoRef.current;
        if (!video || cancelled) { resolve("failed"); return; }

        if (!Hls.isSupported()) {
          // Safari 네이티브
          video.src = url;
          video.addEventListener("playing", () => {
            if (!cancelled) setStatus("playing");
            resolve("playing");
          }, { once: true });
          video.addEventListener("error", () => resolve("failed"), { once: true });
          video.play().catch(() => resolve("failed"));
          return;
        }

        const hls = new Hls({ enableWorker: true, lowLatencyMode: true });
        hlsRef.current = hls;
        hls.loadSource(url);
        hls.attachMedia(video);

        hls.on(Hls.Events.MANIFEST_PARSED, () => {
          video.play().catch(() => {});
        });

        video.addEventListener("playing", () => {
          if (!cancelled) setStatus("playing");
          resolve("playing");
        }, { once: true });

        hls.on(Hls.Events.ERROR, (_event, data) => {
          if (!data.fatal) return;
          hls.destroy();
          hlsRef.current = null;
          resolve("failed");
        });
      });
    }

    run();

    return () => {
      cancelled = true;
      clearTimeout(retryTimer);
      hlsRef.current?.destroy();
      hlsRef.current = null;
      if (videoRef.current) videoRef.current.src = "";
    };
  }, [url, minSegments]);

  return (
    <div className={className}>
      <video
        ref={videoRef}
        autoPlay
        playsInline
        muted
        className="w-full h-full bg-black rounded-md"
      />
      {status === "waiting" && (
        <div className="absolute inset-0 flex items-center justify-center bg-black/50 rounded-md">
          <span className="text-white text-sm">버퍼링 중...</span>
        </div>
      )}
      {status === "error" && (
        <div className="absolute inset-0 flex items-center justify-center bg-black/80 rounded-md">
          <span className="text-red-400 text-sm">{errorMsg}</span>
        </div>
      )}
    </div>
  );
}
