import { ImageResponse } from "next/og";

// Image metadata
export const size = {
  width: 256,
  height: 256,
};

export const contentType = "image/png";

// Image generation
export default function Icon() {
  return new ImageResponse(
    (
      // ImageResponse JSX element
      <div
        style={{
          fontSize: 256,
          width: "100%",
          height: "100%",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          color: "white",
        }}
      >
        💽
      </div>
    ),
    { ...size },
  );
}
