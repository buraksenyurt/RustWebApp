import {getCall} from "./utils";
import {Url} from "./url";
import {WorkItems} from "../interfaces/workItems";

export default async function getAll() {
    await getCall<WorkItems>(
        new Url().getAll, 200
    )
}