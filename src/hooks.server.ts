export async function handle({ event, resolve }) {
	// console.log("Loading layout...");
	// fetch("http://localhost:8000/slow/3000");
	// console.log("Layout loaded.");
	// const response = await resolve(event);
	// return response;
	return resolve(event);
}
