import { useEffect, useRef, useState } from "react";

interface WebRTCPlayerProps {
  url: string; // WHEP endpoint, e.g. http://localhost:8889/stream1/whep
  className?: string;
}

export function WebRTCPlayer({ url, className }: WebRTCPlayerProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const pcRef = useRef<RTCPeerConnection | null>(null);
  const [status, setStatus] = useState<"connecting" | "playing" | "error">("connecting");
  const [errorMsg, setErrorMsg] = useState("");

  useEffect(() => {
    let cancelled = false;

    async function connect() {
      setStatus("connecting");
      setErrorMsg("");

      const pc = new RTCPeerConnection({
        iceServers: [{ urls: "stun:stun.l.google.com:19302" }],
      });
      pcRef.current = pc;

      pc.addTransceiver("video", { direction: "recvonly" });
      pc.addTransceiver("audio", { direction: "recvonly" });

      pc.ontrack = (event) => {
        if (videoRef.current && event.streams[0]) {
          videoRef.current.srcObject = event.streams[0];
          if (!cancelled) setStatus("playing");
        }
      };

      pc.onconnectionstatechange = () => {
        if (cancelled) return;
        if (pc.connectionState === "failed" || pc.connectionState === "disconnected") {
          setStatus("error");
          setErrorMsg("연결이 끊어졌습니다");
        }
      };

      try {
        const offer = await pc.createOffer();
        await pc.setLocalDescription(offer);

        // ICE gathering이 완료될 때까지 대기
        const completeOffer = await waitForIceGathering(pc);

        const resp = await fetch(url, {
          method: "POST",
          headers: { "Content-Type": "application/sdp" },
          body: completeOffer,
        });

        if (!resp.ok) {
          const body = await resp.text().catch(() => "");
          throw new Error(`WHEP ${resp.status}: ${body}`);
        }

        const answerSdp = await resp.text();
        await pc.setRemoteDescription({
          type: "answer",
          sdp: answerSdp,
        });
      } catch (e) {
        if (!cancelled) {
          setStatus("error");
          setErrorMsg(e instanceof Error ? e.message : "연결 실패");
        }
      }
    }

    connect();

    return () => {
      cancelled = true;
      if (pcRef.current) {
        pcRef.current.close();
        pcRef.current = null;
      }
      if (videoRef.current) {
        videoRef.current.srcObject = null;
      }
    };
  }, [url]);

  return (
    <div className={className}>
      <video
        ref={videoRef}
        autoPlay
        playsInline
        muted
        className="w-full h-full bg-black rounded-md"
      />
      {status === "connecting" && (
        <div className="absolute inset-0 flex items-center justify-center bg-black/50 rounded-md">
          <span className="text-white text-sm">Connecting (WebRTC)...</span>
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

function waitForIceGathering(pc: RTCPeerConnection): Promise<string> {
  return new Promise((resolve) => {
    if (pc.iceGatheringState === "complete") {
      resolve(pc.localDescription!.sdp);
      return;
    }
    pc.onicegatheringstatechange = () => {
      if (pc.iceGatheringState === "complete") {
        resolve(pc.localDescription!.sdp);
      }
    };
    // 타임아웃: 5초 후 현재 상태로 진행
    setTimeout(() => {
      resolve(pc.localDescription!.sdp);
    }, 5000);
  });
}
