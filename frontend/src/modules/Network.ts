import axios from "axios";

const axios_inst = axios.create({
    headers: {
        "Content-Type": "application/json",
    },
});

export const HttpGet = axios_inst.get;
export const HttpPost = axios.post;
