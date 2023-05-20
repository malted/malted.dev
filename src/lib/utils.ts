declare type Time = [number, number];

function tzParse(tz: string): Time {
	return new Intl.DateTimeFormat("en-GB", {
		timeZone: tz,
		hour: "2-digit",
		minute: "2-digit",
		hour12: false,
	})
		.format(new Date())
		.split(":")
		.map((x) => Number(x)) as Time;
}

export function isWorkingHours(tz: string): boolean {
	const [hours] = tzParse(tz);

	// Check if it's between 9am and 5pm
	return hours >= 9 && hours < 17;
}

export function timeUntilOpen(tz: string): string {
	const [hours, minutes] = tzParse(tz);

	if (hours >= 9 && hours < 17) return "a bit";

	let hoursUntilOpen = hours < 9 ? 9 - hours : 24 - hours + 9;
	if (minutes === 0) {
		return `${hoursUntilOpen} hour${hoursUntilOpen !== 1 ? "s" : ""}`;
	} else {
		return `${--hoursUntilOpen} hour${
			hoursUntilOpen !== 1 ? "s" : ""
		} and ${60 - minutes} minute${60 - minutes !== 1 ? "s" : ""}`;
	}
}

export function calculatePointsOnArc(
	numPoints: number,
	arcAngle: number,
	radius: number,
	angleOffset = 0
) {
	// Convert arcAngle from degrees to radians
	let arcAngleInRadians = arcAngle * (Math.PI / 180);
	// Calculate the angular separation between each point
	let angleStep = -arcAngleInRadians / (numPoints - 1);
	let points = [];

	for (let i = 0; i < numPoints; i++) {
		let theta = i * angleStep + angleOffset * (Math.PI / 180); // Angle for each point, with the offset applied
		let tangentTheta = theta + Math.PI / 2; // Angle of tangent at each point

		let x = radius * Math.cos(theta); // x-coordinate
		let y = radius * Math.sin(theta); // y-coordinate

		// Convert tangentTheta from radians to degrees
		let tangentThetaInDegrees = tangentTheta * (180 / Math.PI);

		// If the tangent angle is negative, add 360 to make it positive.
		if (tangentThetaInDegrees < 0) {
			tangentThetaInDegrees += 360;
		}

		points.push({
			x: x - radius,
			y: -y,
			tangentAngle: 90 - tangentThetaInDegrees,
		});
	}

	return points;
}

export function shuffleArray(a: [any]): [any] {
	let j, x, i;
	for (i = a.length - 1; i > 0; i--) {
		j = Math.floor(Math.random() * (i + 1));
		x = a[i];
		a[i] = a[j];
		a[j] = x;
	}
	return a;
}

export function degreesToRadians(degrees: number): number {
	return (degrees * Math.PI) / 180;
}
export function radiansToDegrees(radians: number): number {
	return (radians * 180) / Math.PI;
}

export function distanceInKmBetweenEarthCoordinates(
	lat1: number,
	lon1: number,
	lat2: number,
	lon2: number
): number {
	var earthRadiusKm = 6371;

	var dLat = degreesToRadians(lat2 - lat1);
	var dLon = degreesToRadians(lon2 - lon1);

	lat1 = degreesToRadians(lat1);
	lat2 = degreesToRadians(lat2);

	var a =
		Math.sin(dLat / 2) * Math.sin(dLat / 2) +
		Math.sin(dLon / 2) *
			Math.sin(dLon / 2) *
			Math.cos(lat1) *
			Math.cos(lat2);
	var c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

	return earthRadiusKm * c;
}
