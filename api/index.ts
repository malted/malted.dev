#!/usr/bin/env deno run --include-files idx.txt

import ms from "https://esm.sh/ms@2.1.3";
import type { Handler } from "https://deno.land/std@0.177.0/http/server.ts";
import * as haversine from "../utils/haversine.ts";

const handler: Handler = async ({ headers }) => {
    let idx = await Deno.readTextFile("./idx.txt");

    const greetings = [
        "Sappenin'",
        "Hullo there",
        "Howdy 🤠"
    ];

    const maltedLocationToken = Deno.env().location_token;
    const maltedLocationRes = await fetch(`https://internal.bank.engineering/malted/api/location?token=${maltedLocationToken}`).then((d) => d.json());
    const [maltedCoords, maltedCity] = maltedLocationRes.message.split("$");
    const [maltedLat, maltedLng] = maltedCoords.split(",");
    const vercelLocation = [headers["x-vercel-ip-latitude"], headers["x-vercel-ip-longitude"]];
    let distance = haversine.distance(maltedLat, maltedLng, vercelLocation[0], vercelLocation[1]) || "a million";
    distance += " miles";

    const replacements = [
        { from: "greeting", to: greetings[Math.floor(Math.random() * greetings.length)] },
        { from: "city", to: maltedCity },
        { from: "distance", to: distance },
        { from: "cta", to: "We should get coffee!" },
    ];

    replacements.forEach((replacement) => {
        idx = idx.replace(`🥺 ${replacement.from} 🥺`, replacement.to);
    });

    const status = 200;
    return new Response(idx, { status });

};

export default handler;
