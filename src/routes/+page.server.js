import jwa from "jwa";
import { SECRET_TOKEN, MAPKIT_P8 } from "$env/static/private";

const { sign } = jwa("ES256");

export async function load() {
	const locationRes = await fetch("https://api.malted.dev/location?token=" + SECRET_TOKEN);
	if (!locationRes.ok) {
		return { locationUrl: null };
	}

	const location = await locationRes.json();

	if (!location || !location.success) {
		return { locationUrl: null };
	}

	const completePath = `/api/v1/snapshot?center=${location.message}&poi=0&t=mutedStandard&size=640x300&scale=1&colorScheme=dark&teamId=P6PV2R9443&keyId=V9WLJ634MM`;
	const signature = sign(completePath, MAPKIT_P8);
	const locationUrl = `https://snapshot.apple-mapkit.com${completePath}&signature=${signature}`;

	// Request the image to cache it, and convert it to a data URL
	let dataUrl;
	try {
		const imageRes = await fetch(locationUrl);
		let buffer = await imageRes.arrayBuffer();
		let imageStr = Buffer.from(buffer).toString("base64");
		dataUrl = "data:image/png;base64," + imageStr;
	} catch (e) {
		console.error(e);
	}

	return { mapImg: dataUrl };
}
