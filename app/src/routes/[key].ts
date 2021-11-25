
async function getLongUrl(key: String): Promise<String> {
	const api = "https://6uy7eq7dpi.execute-api.us-east-1.amazonaws.com/prod/get";
	const longUrl: Response = await fetch(api, {
		method: "POST",
		body: JSON.stringify({
			key: Number(key),
		}),
	});

	const url = JSON.parse(await longUrl.text()).url
	console.log(url);

	return `https://${url}`;
}

export async function get({ params }) {
	const { key } = params;
	const redirect: String = await getLongUrl(key);

	return {
		status: 302,
		headers: {
			'Location': redirect,
		}
	};
}