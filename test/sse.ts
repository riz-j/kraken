import EventSource from "eventsource";

const es = new EventSource("http://localhost:2900/sse");

es.onmessage = function(event) {
  console.log(event);
}

es.onerror = function(error) {
  console.error(error);
}