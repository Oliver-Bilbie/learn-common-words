import { ReactElement } from "react";

export type ResponseType = {
    message?: string;
    body?: string;
};

export type SortType = { value: string; sort: number };

export type ButtonType = {
    label: string;
    icon: ReactElement;
    onClick: () => void;
};
