import React, { useState, useEffect, ReactElement, useRef } from "react";
import { Box, Button, Heading, Text, Spinner } from "grommet";
import { Close } from "grommet-icons";

import PopupMenu from "../PopupMenu/PopupMenu";
import ProgressMeter from "../ProgressMeter/ProgressMeter";
import { callApi } from "../../helpers/callApi";
import { BASE_ENDPOINT } from "../../config";
import { SortType } from "../../types";
import Confetti from "react-confetti";

const Game: React.FC = (): React.ReactElement => {
  const [wordLimit, setWordLimit] = useState(50);
  const [progress, setProgress] = useState(0);
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

  const confetti = useRef(false);

  useEffect(() => {
    // Load initial words
    callApi(`${BASE_ENDPOINT}/${wordLimit}`, handleApiResponse);
  }, []);

  // Callback function to handle the response from API calls
  const handleApiResponse = (response): void => {
    if (response.message) {
      // If an error message is returned, display it
      setAlert({ message: response.message, show: true });
    } else {
      setWords(response.body);
      setLoading(false);
    }
  };

  // Called when the user selects an answer
  const handleSelection = (selection: string): void => {
    // Update the player's progress
    const newProgress =
      selection === words.correct
        ? progress + 1
        : progress >= 5
        ? progress - 5
        : 0;
    setProgress(newProgress);

    // Update the player's word limit
    const newWordLimit = 50 + 50 * Math.floor(newProgress / 10);
    setWordLimit(newWordLimit);

    // Fire confetti if the word limit has increased this turn
    if (newWordLimit > wordLimit) {
      confetti.current = true;
    }

    // Display the answer to the player
    setAnswer({
      ...words,
      show: true,
      color: selection === words.correct ? "green" : "red",
    });

    // Load the next set of words in the background
    setLoading(true);
    callApi(`${BASE_ENDPOINT}/${newWordLimit}`, handleApiResponse);
  };

  return (
    <Box>
      <Confetti
        numberOfPieces={confetti.current ? 1500 : 0}
        gravity={0.1}
        recycle={false}
        /* eslint-disable @typescript-eslint/no-explicit-any */
        onConfettiComplete={(confetti_obj: any): void => {
          confetti.current = false;
          confetti_obj.reset();
        }}
      />
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
              <Heading textAlign="center" margin={{ bottom: "xsmall" }}>
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
                onClick={(): void => setAnswer({ ...answer, show: false })}
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
                  .sort((a: SortType, b: SortType): number => a.sort - b.sort)
                  .map(
                    ({ value }: { value: string }): ReactElement => (
                      <Button
                        label={value}
                        onClick={(): void => handleSelection(value)}
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
                onClick: (): void => setAlert({ ...alert, show: false }),
              },
            ]}
          />
        )}
      </Box>
    </Box>
  );
};

export default Game;
