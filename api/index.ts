#!/usr/bin/env deno run --include-files idx.txt

import ms from "https://esm.sh/ms@2.1.3";
import type { Handler } from "https://deno.land/std@0.177.0/http/server.ts";
import * as haversine from "../utils/haversine.ts";

const handler: Handler = async ({ headers }) => {
    let idx = await Deno.readTextFile("./idx.txt");

    const greetings = [
        "Sappenin'?",
        "Hi!",
        "Howdy 🤠.",
        "G'day!",
        "Salutations.",
        "How do you do?",
    ];

    const maltedLocationToken = Deno.env.get("location_token");
    const maltedLocationRes = await fetch(`https://internal.bank.engineering/malted/api/location?token=${maltedLocationToken}`).then((d) => d.json());
    const [maltedLat, maltedLng] = maltedLocationRes.location.coords.split(",");
    const maltedCity = maltedLocationRes.location.city;
    const maltedCountry = maltedLocationRes.location.country;
    const maltedLocationTimestamp = new Date(maltedLocationRes.location.timestamp).getTime();
    const [visitorLat, visitorLng] = [headers.get("cf-iplatitude"), headers.get("cf-iplongitude")];

    let distance = haversine.distance(maltedLat, maltedLng, visitorLat, visitorLng); 
    distance = Math.round(distance);
    const distanceRaw = distance;
    distance ||= "a million";
    distance += " miles";

    console.log(maltedLat, maltedLng, maltedCity, maltedCountry, headers);

    let locationSentence = `I'm in ${maltedCity}, in ${maltedCountry}; so we're around ${distance} away from each other right now. Next time I'm around, we should hang out!`;
    if (distanceRaw < 5) {
        locationSentence = `I'm in ${maltedCity}, just like you! This is awesome. Since we're so close, we should grab coffee.`;
    }

    const replacements = [
        { from: "greeting", to: greetings[Math.floor(Math.random() * greetings.length)] },
        { from: "timestamp", to: ms(maltedLocationTimestamp) },
        { from: "location", to: locationSentence },
    ];

    replacements.forEach((replacement) => {
        idx = idx.replace(`🥺 ${replacement.from} 🥺`, replacement.to);
    });

    const status = 200;
    return new Response(idx, { status });
};

export default handler;

