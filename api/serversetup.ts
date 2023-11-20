#!/usr/bin/env deno run --include-files serversetup.sh

import type { Handler } from "https://deno.land/std@0.177.0/http/server.ts";

const handler: Handler = async ({ url }) => {
	const [username, ip] = new URL(url).pathname.split("/").slice(2);
    let idx = await Deno.readTextFile("./serversetup.sh");

    const replacements = [
        { from: "DESIRED_REMOTE_USERNAME", to: username },
        { from: "REMOTE_IP", to: ip },
    ];

    replacements.forEach((replacement) => {
        idx = idx.replace(`🥺 ${replacement.from} 🥺`, replacement.to);
    });

	console.log(idx);

	const status = 200;
    return new Response(idx, { status });
};

export default handler;

