import React from "react";
import styled from "styled-components";
import { Title, bg, mainSearch, mainImg } from "../images";
import { Link } from "react-router-dom";
const Intro = () => {
  return (
    <S.Container>
      <S.Titleimg src={Title} alt="title" />
      <S.Row>
        <Link to="/inputImage">
          <S.LinkBox src={mainImg} alt="img" />
        </Link>

        <Link to="/research">
          <S.LinkBox2 src={mainSearch} alt="research" />
        </Link>
      </S.Row>
    </S.Container>
  );
};

const S = {
  Container: styled.div`
    height: 100vh;
    padding-inline: 120px;
    background-image: url(${bg});
    background-repeat: no-repeat;
    background-size: cover;
    overflow-x: hidden;
  `,
  Titleimg: styled.img`
    margin-block: 30px;
  `,
  Row: styled.div`
    display: flex;
  `,

  LinkBox: styled.img`
    cursor: pointer;
    width: 732px;
    height: 464px;
    margin-right: 16px;
  `,

  LinkBox2: styled.img`
    cursor: pointer;
    width: 516px;
    height: 464px;
  `,
};

export default Intro;
