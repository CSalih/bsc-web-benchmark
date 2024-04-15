import { onCLS, onFID, onLCP, onINP, onFCP, onTTFB } from 'https://unpkg.com/web-vitals@3.5.2/dist/web-vitals.js?module';


function sendToAnalyticsServer(metric) {
    const body = JSON.stringify({
        type: "web-vitals-v1",
        project: "@morandell/sales-representative",
        timestamp: new Date().toISOString(),
        href: location ? location.href : null,
        metadata: {
            referrer: document ? document.referrer : null,
            userAgent: navigator ? navigator.userAgent : null,
            memory: navigator ? navigator.deviceMemory : undefined,
            cpus: navigator ? navigator.hardwareConcurrency : undefined,
            webdriver: navigator ? navigator.webdriver : undefined,
            connection: navigator?.connection ? {
                type: navigator.connection.type,
                downlink: navigator.connection.downlink,
                downlinkMax: navigator.connection.downlinkMax,
                effectiveType: navigator.connection.effectiveType,
                saveData: navigator.connection.saveData,
                rtt: navigator.connection.rtt,
            } ?? undefined : undefined,
        },
        data: metric,
    });
    const url = "http://localhost:3000/api/event";
    if (navigator.sendBeacon) {
        navigator.sendBeacon(url, body);
    } else {
        fetch(url, {
            body,
            method: "POST",
            keepalive: true,
            headers: { "Content-Type": "application/json" },
        });
    }
}

onTTFB(sendToAnalyticsServer);
onFCP(sendToAnalyticsServer);
onCLS(sendToAnalyticsServer);
onFID(sendToAnalyticsServer);
onLCP(sendToAnalyticsServer);
onINP(sendToAnalyticsServer);