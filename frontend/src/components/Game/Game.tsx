import React, { useState, useEffect, ReactElement } from "react";
import { Box, Button, Heading, Text, Spinner } from "grommet";
import { Close } from "grommet-icons";

import PopupMenu from "../PopupMenu/PopupMenu";
import ProgressMeter from "../ProgressMeter/ProgressMeter";
import { callApi } from "../../helpers/callApi";
import { BASE_ENDPOINT } from "../../config";
import { SortType } from "../../types";
import ReactCanvasConfetti from "react-canvas-confetti";

const Game: React.FC = (): React.ReactElement => {
    const [wordLimit, setWordLimit] = useState(50);
    const [progress, setProgress] = useState(0);
    const [confetti, setConfetti] = useState(false);
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
        color: "green",
    });

    useEffect(() => {
        // Load initial words
        callApi(`${BASE_ENDPOINT}/${wordLimit}`, handleApiResponse);
    }, []);

    const fireConfetti = (): void => {
        setConfetti(!confetti);
    };

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

        if (selection === words.correct) {
            fireConfetti();
        }
        const newProgress =
            selection === words.correct
                ? progress + 1
                : progress >= 5
                ? progress - 5
                : 0;

        setProgress(newProgress);
        setWordLimit(50 + 50 * Math.floor(newProgress / 10));

        // Display the answer
        setAnswer({
            ...words,
            show: true,
            color: selection === words.correct ? "green" : "red",
        });
        // Load new words in the background
        setLoading(true);
        callApi(`${BASE_ENDPOINT}/${wordLimit}`, handleApiResponse);
    };

    return (
        <Box fill>
            <Box
                width="medium"
                height="600px"
                direction="column"
                align="center"
                alignSelf="center"
                alignContent="center"
                justify="center"
                background="light-2"
                elevation="small"
                margin="small"
                round
            >
                <ProgressMeter progress={progress} wordLimit={wordLimit} />
                <Box height="250px" align="center" justify="center">
                    {answer.show ? (
                        <Box align="center">
                            <Heading
                                textAlign="center"
                                margin={{ bottom: "xsmall" }}
                            >
                                {answer.word}
                            </Heading>
                            <Heading
                                textAlign="center"
                                margin={{ top: "xsmall" }}
                                color={answer.color}
                            >
                                {answer.correct}
                            </Heading>
                            <Button
                                label="Next"
                                onClick={(): void =>
                                    setAnswer({ ...answer, show: false })
                                }
                            />
                        </Box>
                    ) : loading ? (
                        <Spinner size="large" pad="small" />
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
                </Box>
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
            <ReactCanvasConfetti fire={confetti} />
        </Box>
    );
};

export default Game;
