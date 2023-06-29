import React from "react";
import styled from "styled-components";
import { Title, bg, mainSearch, mainImg } from "../images";
import { Link } from "react-router-dom";
const Intro = () => {
  return (
    <S.Container>
      <S.Titleimg src={Title} alt="title" />
      <br />
      <Link to="/inputImage">
        <S.LinkText src={mainImg} alt="img" />
      </Link>

      <Link to="/research">
        <S.LinkText2 src={mainSearch} alt="research" />
      </Link>
    </S.Container>
  );
};

const S = {
  Container: styled.div`
    padding-inline: 120px;
    padding-top: 5%;
    background-image: url(${bg});
    background-repeat: no-repeat;
    background-size: cover;
    width: 100%;
  `,
  Titleimg: styled.img`
    margin-bottom: 30px;
  `,

  tempArea: styled.div`
    min-height: 10vh;
  `,
  LinkText: styled.img`
    cursor: pointer;
    width: 732px;
    height: 464px;
    margin-right: 16px;
  `,

  LinkText2: styled.img`
    cursor: pointer;
    width: 516px;
    height: 464px;
  `,
};

export default Intro;
