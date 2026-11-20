import { Url } from "./url";
import { postCall } from "./utils";

export default async function logout() {
	return postCall<void>(new Url().logout, {}, 204);
}
