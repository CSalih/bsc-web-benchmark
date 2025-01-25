import { onCLS, onFID, onLCP, onINP, onFCP, onTTFB } from "web-vitals";

const url = new URL(import.meta.url);
const project = url.searchParams.get("project") ?? "unknown";
const baseUrl = `${url.origin}/api/v1/event`;

function sendToAnalyticsServer(metric) {
  const body = JSON.stringify({
    type: "web-vitals/v1",
    project,
    timestamp: new Date().toISOString(),
    href: location ? location.href : null,
    metadata: {
      referrer: document ? document.referrer : null,
      userAgent: navigator ? navigator.userAgent : null,
      memory: navigator ? navigator.deviceMemory : undefined,
      cpus: navigator ? navigator.hardwareConcurrency : undefined,
      webdriver: navigator ? navigator.webdriver : undefined,
      connection: navigator?.connection
        ? {
            type: navigator.connection.type,
            downlink: navigator.connection.downlink,
            downlinkMax: navigator.connection.downlinkMax,
            effectiveType: navigator.connection.effectiveType,
            saveData: navigator.connection.saveData,
            rtt: navigator.connection.rtt,
          } ?? undefined
        : undefined,
    },
    data: metric,
  });
  if (navigator.sendBeacon) {
    navigator.sendBeacon(baseUrl, body);
  } else {
    fetch(baseUrl, {
      body,
      method: "POST",
      keepalive: true,
      headers: { "Content-Type": "application/json" },
    });
  }
}

function persistInMemory(metric) {
  window.webVitals = window.webVitals || [];
  window.webVitals.push(metric);
}

onTTFB(persistInMemory);
onFCP(persistInMemory);
onCLS(persistInMemory);
onFID(persistInMemory);
onLCP(persistInMemory);
onINP(persistInMemory);
