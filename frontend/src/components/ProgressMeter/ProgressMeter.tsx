import React from "react";
import { Box, Meter, Stack, Text } from "grommet";

interface ProgressMeterProps {
  progress: number;
  wordLimit: number;
}

const ProgressMeter: React.FC<ProgressMeterProps> = ({
  progress,
  wordLimit,
}): React.ReactElement => {
  return (
    <Stack anchor="center">
      <Meter
        value={(progress % 10) * 10}
        type="circle"
        color="brand"
        background="background"
        size="250px"
      />
      <Box width="medium" align="center">
        <Text>Learning the</Text>
        <Text weight="bold" size="xlarge">
          {wordLimit}
        </Text>
        <Text>most common words</Text>
      </Box>
    </Stack>
  );
};

export default ProgressMeter;
