import React, { useState, useEffect, ReactElement } from "react";
import { Box, Button, Heading, Text, Spinner } from "grommet";
import { Close } from "grommet-icons";

import PopupMenu from "../PopupMenu/PopupMenu";
import { callApi } from "../../helpers/callApi";
import { BASE_ENDPOINT } from "../../config";
import { SortType } from "../../types";

const Game: React.FC = (): React.ReactElement => {
    // const [wordLimit, setWordLimit] = useState(100);
    const worldLimit = 100;
    const [loading, setLoading] = useState(true);
    const [alert, setAlert] = useState({ message: "", show: false });
    const [words, setWords] = useState({
        word: "",
        correct: "",
        decoy_1: "",
        decoy_2: "",
    });
    const [answer, setAnswer] = useState({
        word: "",
        correct: "",
        decoy_1: "",
        decoy_2: "",
        show: false,
    });

    useEffect(() => {
        // Load initial words
        callApi(`${BASE_ENDPOINT}/${worldLimit}`, handleApiResponse);
    }, []);

    const handleApiResponse = (response): void => {
        // Handles the response from the API
        if (response.message) {
            setAlert({ message: response.message, show: true });
        } else {
            setWords(response.body);
            setLoading(false);
        }
    };

    const handleSelection = (selection: string): void => {
        // Called when the user selects an answer
        console.log(selection);

        // Display the answer
        setAnswer({ ...words, show: true });
        // Load new words in the background
        setLoading(true);
        callApi(`${BASE_ENDPOINT}/${worldLimit}`, handleApiResponse);
    };

    return (
        <Box
            width="medium"
            height="medium"
            direction="column"
            align="center"
            alignContent="center"
            justify="center"
            background="light-2"
            elevation="small"
            margin="small"
            round
        >
            {loading ? (
                <Spinner size="large" pad="small" />
            ) : (
                <>
                    {answer.show ? (
                        <Box align="center">
                            <Heading textAlign="center">{answer.word}</Heading>
                            <Heading textAlign="center" color="green">
                                {answer.correct}
                            </Heading>
                            <Button
                                label="Next"
                                onClick={(): void =>
                                    setAnswer({ ...answer, show: false })
                                }
                            />
                        </Box>
                    ) : (
                        <Box align="center">
                            <Heading textAlign="center">{words.word}</Heading>
                            <Box gap="small">
                                {[words.correct, words.decoy_1, words.decoy_2]
                                    .map(
                                        (value: string): SortType => ({
                                            value,
                                            sort: Math.random(),
                                        })
                                    )
                                    .sort(
                                        (a: SortType, b: SortType): number =>
                                            a.sort - b.sort
                                    )
                                    .map(
                                        ({
                                            value,
                                        }: {
                                            value: string;
                                        }): ReactElement => (
                                            <Button
                                                label={value}
                                                onClick={(): void =>
                                                    handleSelection(value)
                                                }
                                                key={`button-${value}`}
                                            />
                                        )
                                    )}
                            </Box>
                        </Box>
                    )}
                </>
            )}
            {alert.show && (
                <PopupMenu
                    body={<Text>{alert.message}</Text>}
                    buttons={[
                        {
                            label: "Close",
                            icon: <Close />,
                            onClick: (): void =>
                                setAlert({ ...alert, show: false }),
                        },
                    ]}
                />
            )}
        </Box>
    );
};

export default Game;
