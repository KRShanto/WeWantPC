import Image1 from "../../public/images/slider/benjamin-lehman-GNyjCePVRs8-unsplash.jpg";
import Image2 from "../../public/images/slider/boicu-andrei-YtDIYzUAG9Q-unsplash.jpg";
import Image3 from "../../public/images/slider/christian-wiediger-WkfDrhxDMC8-unsplash.jpg";
import Image4 from "../../public/images/slider/daniel-hatcher-zPHftoPajis-unsplash.jpg";
import Image5 from "../../public/images/slider/clement-helardot-95YRwf6CNw8-unsplash.jpg";
import Image6 from "../../public/images/slider/olivier-collet-JMwCe3w7qKk-unsplash.jpg";

import RightArrow from "../../public/images/right-arrow.svg";
import LeftArrow from "../../public/images/left-arrow.svg";
import React, { useState } from "react";
import Image from "next/image";

export default function ImageSlider() {
  const [current, setCurrent] = useState(0);
  const images = [Image1, Image2, Image3, Image4, Image5, Image6];
  const length = images.length;

  function nextSlide() {
    setCurrent(current === length - 1 ? 0 : current + 1);
  }

  function prevSlide() {
    setCurrent(current === 0 ? length - 1 : current - 1);
  }

  return (
    <div className="image-slider">
      <button className="left-arrow" onClick={prevSlide}>
        <Image src={LeftArrow} alt="left-arrow" />
      </button>

      <Image src={images[current]} alt="image" />

      <button className="right-arrow" onClick={nextSlide}>
        <Image src={RightArrow} alt="right-arrow" />
      </button>
    </div>
  );
}
